use byteorder::{BigEndian, ByteOrder, WriteBytesExt};
use bytes::Bytes;
use futures::channel::{mpsc, oneshot};
use futures::Stream;
use futures::{Future, task::Poll};
use super::range_set::{Range, RangeSet};
use std::cmp::{max, min};
use std::fs;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;

use futures::channel::mpsc::unbounded;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

const MINIMUM_DOWNLOAD_SIZE: usize = 1024 * 16;
const INITIAL_DOWNLOAD_SIZE: usize = 1024 * 16;
const INITIAL_PING_TIME_ESTIMATE_SECONDS: f64 = 0.5;
const MAXIMUM_ASSUMED_PING_TIME_SECONDS: f64 = 1.5;

pub const READ_AHEAD_BEFORE_PLAYBACK_SECONDS: f64 = 1.0;
pub const READ_AHEAD_BEFORE_PLAYBACK_ROUNDTRIPS: f64 = 2.0;
pub const READ_AHEAD_DURING_PLAYBACK_SECONDS: f64 = 5.0;
pub const READ_AHEAD_DURING_PLAYBACK_ROUNDTRIPS: f64 = 10.0;

const PREFETCH_THRESHOLD_FACTOR: f64 = 4.0;
const FAST_PREFETCH_THRESHOLD_FACTOR: f64 = 1.5;
const MAX_PREFETCH_REQUESTS: usize = 4;

pub enum AudioFile {
    Cached(fs::File),
    Streaming(AudioFileStreaming),
}

pub enum AudioFileOpen {
    Cached(Option<fs::File>),
    Streaming(AudioFileOpenStreaming),
}

pub struct AudioFileOpenStreaming {
    // session: Session,
    // initial_data_rx: Option<ChannelData>,
    initial_data_length: Option<usize>,
    initial_request_sent_time: Instant,
    // headers: ChannelHeaders,
    // file_id: FileId,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
    streaming_data_rate: usize,
}

pub struct AudioFileStreaming {
    read_file: fs::File,
    position: u64,
    stream_loader_command_tx: mpsc::UnboundedSender<StreamLoaderCommand>,
    shared: Arc<AudioFileShared>,
}

enum StreamLoaderCommand {
    Fetch(Range),       // signal the stream loader to fetch a range of the file
    RandomAccessMode(), // optimise download strategy for random access
    StreamMode(),       // optimise download strategy for streaming
    Close(),            // terminate and don't load any more data
}

struct AudioFileDownloadStatus {
    requested: RangeSet,
    downloaded: RangeSet,
}

#[derive(Copy, Clone)]
enum DownloadStrategy {
    RandomAccess(),
    Streaming(),
}


struct PartialFileData {
    offset: usize,
    data: Bytes,
}

enum ReceivedData {
    ResponseTimeMs(usize),
    Data(PartialFileData),
}

struct AudioFileFetchDataReceiver {
    shared: Arc<AudioFileShared>,
    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    // data_rx: ChannelData,
    initial_data_offset: usize,
    initial_request_length: usize,
    data_offset: usize,
    request_length: usize,
    request_sent_time: Option<Instant>,
    measure_ping_time: bool,
}

struct AudioFileShared {
    // file_id: FileId,
    file_size: usize,
    stream_data_rate: usize,
    cond: Condvar,
    download_status: Mutex<AudioFileDownloadStatus>,
    download_strategy: Mutex<DownloadStrategy>,
    number_of_open_requests: AtomicUsize,
    ping_time_ms: AtomicUsize,
    read_position: AtomicUsize,
}

#[derive(Clone)]
pub struct StreamLoaderController {
    channel_tx: Option<mpsc::UnboundedSender<StreamLoaderCommand>>,
    stream_shared: Option<Arc<AudioFileShared>>,
    file_size: usize,
}

impl StreamLoaderController {
    pub fn len(&self) -> usize {
        return self.file_size;
    }

    pub fn range_available(&self, range: Range) -> bool {
        if let Some(ref shared) = self.stream_shared {
            let download_status = shared.download_status.lock().unwrap();
            if range.length
                <= download_status
                    .downloaded
                    .contained_length_from_value(range.start)
            {
                return true;
            } else {
                return false;
            }
        } else {
            if range.length <= self.len() - range.start {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn ping_time_ms(&self) -> usize {
        if let Some(ref shared) = self.stream_shared {
            return shared.ping_time_ms.load(atomic::Ordering::Relaxed);
        } else {
            return 0;
        }
    }

    fn send_stream_loader_command(&mut self, command: StreamLoaderCommand) {
        if let Some(ref mut channel) = self.channel_tx {
            // ignore the error in case the channel has been closed already.
            let _ = channel.unbounded_send(command);
        }
    }

    pub fn fetch(&mut self, range: Range) {
        // signal the stream loader to fetch a range of the file
        self.send_stream_loader_command(StreamLoaderCommand::Fetch(range));
    }

    pub fn fetch_blocking(&mut self, mut range: Range) {
        // signal the stream loader to tech a range of the file and block until it is loaded.

        // ensure the range is within the file's bounds.
        if range.start >= self.len() {
            range.length = 0;
        } else if range.end() > self.len() {
            range.length = self.len() - range.start;
        }

        self.fetch(range);

        if let Some(ref shared) = self.stream_shared {
            let mut download_status = shared.download_status.lock().unwrap();
            while range.length
                > download_status
                    .downloaded
                    .contained_length_from_value(range.start)
            {
                download_status = shared
                    .cond
                    .wait_timeout(download_status, Duration::from_millis(1000))
                    .unwrap()
                    .0;
                if range.length
                    > (download_status
                        .downloaded
                        .union(&download_status.requested)
                        .contained_length_from_value(range.start))
                {
                    // For some reason, the requested range is neither downloaded nor requested.
                    // This could be due to a network error. Request it again.
                    // We can't use self.fetch here because self can't be borrowed mutably, so we access the channel directly.
                    if let Some(ref mut channel) = self.channel_tx {
                        // ignore the error in case the channel has been closed already.
                        let _ = channel.unbounded_send(StreamLoaderCommand::Fetch(range));
                    }
                }
            }
        }
    }

    pub fn fetch_next(&mut self, length: usize) {
        let range: Range = if let Some(ref shared) = self.stream_shared {
            Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            }
        } else {
            return;
        };
        self.fetch(range);
    }

    pub fn fetch_next_blocking(&mut self, length: usize) {
        let range: Range = if let Some(ref shared) = self.stream_shared {
            Range {
                start: shared.read_position.load(atomic::Ordering::Relaxed),
                length: length,
            }
        } else {
            return;
        };
        self.fetch_blocking(range);
    }

    pub fn set_random_access_mode(&mut self) {
        // optimise download strategy for random access
        self.send_stream_loader_command(StreamLoaderCommand::RandomAccessMode());
    }

    pub fn set_stream_mode(&mut self) {
        // optimise download strategy for streaming
        self.send_stream_loader_command(StreamLoaderCommand::StreamMode());
    }

    pub fn close(&mut self) {
        // terminate stream loading and don't load any more data for this file.
        self.send_stream_loader_command(StreamLoaderCommand::Close());
    }
}

impl AudioFileOpenStreaming {
    fn finish(&mut self, size: usize) -> AudioFileStreaming {
        let shared = Arc::new(AudioFileShared {
            // file_id: self.file_id,
            file_size: size,
            stream_data_rate: self.streaming_data_rate,
            cond: Condvar::new(),
            download_status: Mutex::new(AudioFileDownloadStatus {
                requested: RangeSet::new(),
                downloaded: RangeSet::new(),
            }),
            download_strategy: Mutex::new(DownloadStrategy::RandomAccess()), // start with random access mode until someone tells us otherwise
            number_of_open_requests: AtomicUsize::new(0),
            ping_time_ms: AtomicUsize::new(0),
            read_position: AtomicUsize::new(0),
        });

        let mut write_file = NamedTempFile::new().unwrap();
        write_file.as_file().set_len(size as u64).unwrap();
        write_file.seek(SeekFrom::Start(0)).unwrap();

        let read_file = write_file.reopen().unwrap();

        // let initial_data_rx = self.initial_data_rx.take().unwrap();
        let initial_data_length = self.initial_data_length.take().unwrap();
        let complete_tx = self.complete_tx.take().unwrap();
        //let (seek_tx, seek_rx) = mpsc::unbounded();
        let (stream_loader_command_tx, stream_loader_command_rx) =
            mpsc::unbounded::<StreamLoaderCommand>();

        // let fetcher = AudioFileFetch::new(
            // // self.session.clone(),
            // shared.clone(),
            // initial_data_rx,
            // self.initial_request_sent_time,
            // initial_data_length,
            // write_file,
            // stream_loader_command_rx,
            // complete_tx,
        // );
        // // self.session.spawn(move |_| fetcher);

        AudioFileStreaming {
            read_file: read_file,

            position: 0,
            //seek: seek_tx,
            stream_loader_command_tx: stream_loader_command_tx,

            shared: shared,
        }
    }
}


struct AudioFileFetch {
    // session: Session,
    shared: Arc<AudioFileShared>,
    output: Option<NamedTempFile>,

    file_data_tx: mpsc::UnboundedSender<ReceivedData>,
    file_data_rx: mpsc::UnboundedReceiver<ReceivedData>,

    stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
    complete_tx: Option<oneshot::Sender<NamedTempFile>>,
    network_response_times_ms: Vec<usize>,
}

impl AudioFileFetch {
    fn new(
        // session: Session,
        shared: Arc<AudioFileShared>,
        // initial_data_rx: ChannelData,
        initial_request_sent_time: Instant,
        initial_data_length: usize,

        output: NamedTempFile,
        stream_loader_command_rx: mpsc::UnboundedReceiver<StreamLoaderCommand>,
        complete_tx: oneshot::Sender<NamedTempFile>,
    ) -> AudioFileFetch {
        let (file_data_tx, file_data_rx) = unbounded::<ReceivedData>();

        {
            let requested_range = Range::new(0, initial_data_length);
            let mut download_status = shared.download_status.lock().unwrap();
            download_status.requested.add_range(&requested_range);
        }

        let initial_data_receiver = AudioFileFetchDataReceiver::new(
            shared.clone(),
            file_data_tx.clone(),
            // initial_data_rx,
            0,
            initial_data_length,
            initial_request_sent_time,
        );

        // session.spawn(move |_| initial_data_receiver);

        AudioFileFetch {
            // session: session,
            shared: shared,
            output: Some(output),

            file_data_tx: file_data_tx,
            file_data_rx: file_data_rx,

            stream_loader_command_rx: stream_loader_command_rx,
            complete_tx: Some(complete_tx),
            network_response_times_ms: Vec::new(),
        }
    }

    fn get_download_strategy(&mut self) -> DownloadStrategy {
        *(self.shared.download_strategy.lock().unwrap())
    }

    fn download_range(&mut self, mut offset: usize, mut length: usize) {
        if length < MINIMUM_DOWNLOAD_SIZE {
            length = MINIMUM_DOWNLOAD_SIZE;
        }

        // ensure the values are within the bounds and align them by 4 for the spotify protocol.
        if offset >= self.shared.file_size {
            return;
        }

        if length <= 0 {
            return;
        }

        if offset + length > self.shared.file_size {
            length = self.shared.file_size - offset;
        }

        if offset % 4 != 0 {
            length += offset % 4;
            offset -= offset % 4;
        }

        if length % 4 != 0 {
            length += 4 - (length % 4);
        }

        let mut ranges_to_request = RangeSet::new();
        ranges_to_request.add_range(&Range::new(offset, length));

        let mut download_status = self.shared.download_status.lock().unwrap();

        ranges_to_request.subtract_range_set(&download_status.downloaded);
        ranges_to_request.subtract_range_set(&download_status.requested);

        for range in ranges_to_request.iter() {
            // let (_headers, data) =
                // request_range(&self.session, self.shared.file_id, range.start, range.length).split();

            download_status.requested.add_range(range);

            // let receiver = AudioFileFetchDataReceiver::new(
                // self.shared.clone(),
                // self.file_data_tx.clone(),
                // data,
                // range.start,
                // range.length,
                // Instant::now(),
            // );

            // self.session.spawn(move |_| receiver);
        }
    }
}

impl AudioFileFetchDataReceiver {
    fn new(
        shared: Arc<AudioFileShared>,
        file_data_tx: mpsc::UnboundedSender<ReceivedData>,
        // data_rx: ChannelData,
        data_offset: usize,
        request_length: usize,
        request_sent_time: Instant,
    ) -> AudioFileFetchDataReceiver {
        let measure_ping_time = shared.number_of_open_requests.load(atomic::Ordering::SeqCst) == 0;

        shared
            .number_of_open_requests
            .fetch_add(1, atomic::Ordering::SeqCst);

        AudioFileFetchDataReceiver {
            shared: shared,
            // data_rx: data_rx,
            file_data_tx: file_data_tx,
            initial_data_offset: data_offset,
            initial_request_length: request_length,
            data_offset: data_offset,
            request_length: request_length,
            request_sent_time: Some(request_sent_time),
            measure_ping_time: measure_ping_time,
        }
    }
}

impl AudioFileFetchDataReceiver {
    fn finish(&mut self) {
        if self.request_length > 0 {
            let missing_range = Range::new(self.data_offset, self.request_length);

            let mut download_status = self.shared.download_status.lock().unwrap();
            download_status.requested.subtract_range(&missing_range);
            self.shared.cond.notify_all();
        }

        self.shared
            .number_of_open_requests
            .fetch_sub(1, atomic::Ordering::SeqCst);
    }
}

// impl Future for AudioFileFetchDataReceiver {

    // fn poll(&mut self) -> Poll<(), ()> {
        // loop {
            // // match self.data_rx.poll() {
            // // }
        // }
    // }
// }
