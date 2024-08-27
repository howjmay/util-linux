// This file is part of the uutils util-linux package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

use clap::{crate_version, Command};
use clap::{Arg, ArgAction};
use nix::errno::Errno;
// use nix::fcntl::SeekWhence;
// use nix::libc::{self, EPIPE};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, ErrorKind};
// use nix::unistd::lseek;
use std::borrow::Borrow;
// use std::fs::{File, OpenOptions};
// use std::io::{self, BufWriter, Read, Seek, SeekFrom};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
use uucore::{error::UResult, format_usage, help_about, help_usage};

const ABOUT: &str = help_about!("dmesg.md");
const USAGE: &str = help_usage!("dmesg.md");

const FILE_KMSG: &str = "/dev/kmsg";

mod options {
    pub const JSON: &str = "json";
}

struct DmesgControl {
    // /* bit arrays -- see include/bitops.h */
    // char levels[ARRAY_SIZE(level_names) / NBBY + 1];
    // char facilities[ARRAY_SIZE(facility_names) / NBBY + 1];

    // struct timeval	lasttime;	/* last printed timestamp */
    // struct tm	lasttm;		/* last localtime */
    // struct timeval	boot_time;	/* system boot time */
    // usec_t		suspended_time;	/* time spent in suspended state */

    // int		action;		/* SYSLOG_ACTION_* */
    // int		method;		/* DMESG_METHOD_* */

    // size_t		bufsize;	/* size of syslog buffer */
    kmsg: Option<File>, // /dev/kmsg file descriptor

    kmsg_first_read: usize, /* initial read() return code */
    // /*
    //  * the kernel will give EINVAL if we do read() on /proc/kmsg with
    //  * length insufficient for the next message. messages may be up to
    //  * PRINTK_MESSAGE_MAX, which is defined as 2048, so we must be
    //  * able to buffer at least that much in one call
    //  */
    kmsg_buf: [u8; 2048], /* buffer to read kmsg data */

                          // usec_t		since;		/* filter records by time */
                          // usec_t		until;		/* filter records by time */

                          // /*
                          //  * For the --file option we mmap whole file. The unnecessary (already
                          //  * printed) pages are always unmapped. The result is that we have in
                          //  * memory only the currently used page(s).
                          //  */
                          // char		*filename;
                          // char		*mmap_buff;
                          // size_t		pagesize;
                          // size_t		ntime_fmts;
                          // unsigned int	time_fmts[2 * __DMESG_TIMEFTM_COUNT];	/* time format */

                          // struct ul_jsonwrt jfmt;		/* -J formatting */

                          // int		indent;		/* due to timestamps if newline */
                          // size_t          caller_id_size;   /* PRINTK_CALLERID max field size */
}

struct Options {
    follow: bool,       // wait for new messages
    end: bool,          // seek to the of buffer
    raw: bool,          // raw mode
    noesc: bool,        // no escape
    fltr_lev: bool,     // filter out by levels[]
    fltr_fac: bool,     // filter out by facilities[]
    decode: bool,       // use "facility: level: " prefix
    pager: bool,        // pipe output into a pager
    color: bool,        // colorize messages
    json: bool,         // JSON output
    force_prefix: bool, // force timestamp and decode prefix on each line
}

fn set_line_buf() -> io::Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let _writer = BufWriter::new(handle);
    Ok(())
}

fn init_kmsg(ctl: &mut DmesgControl, opts: &Options) -> io::Result<()> {
    match File::open(FILE_KMSG) {
        Ok(fd) => {
            ctl.kmsg = Some(fd);
        }
        Err(e) => {
            ctl.kmsg = None;
            return Err(e);
        }
    };

    let mut reader = BufReader::new(ctl.kmsg.as_ref().unwrap());
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => println!("End of file or connection closed"),
        Ok(_) => println!("Read line: {}", line),
        Err(ref e) if e.kind() == ErrorKind::BrokenPipe => {
            eprintln!("Error: Broken pipe (EPIPE) detected while reading");
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
    }

    Ok(())
}

// fn read_kmsg_one(ctl: &mut DmesgControl) -> io::Result<usize> {
//     let mut size: usize;

//     /* kmsg returns EPIPE if record was modified while reading */
//     loop {
//         size = ctl.kmsg.borrow().read(&mut ctl.kmsg_buf)?;
//         if size < 0 && Errno::last() == Errno::EPIPE {
//             continue;
//         }
//         break;
//     }

//     Ok(size)
// }

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let matches: clap::ArgMatches = uu_app().try_get_matches_from(args)?;
    let opt_json = matches.get_flag(options::JSON);

    Ok(())
}

pub fn uu_app() -> Command {
    Command::new(uucore::util_name())
        .version(crate_version!())
        .about(ABOUT)
        .override_usage(format_usage(USAGE))
        .infer_long_args(true)
}
