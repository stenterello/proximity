use esp_idf_hal::gpio::{Gpio13, Output, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::wifi_promiscuous_cb_t;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicI8;
use esp_idf_sys::{self as _, EspError};

fn starts(_val: ()) -> () {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
}

#[derive(Default)]
struct NeverBeen {
    part_of_me: Option<Peripherals>,
    this_time: Option<EspSystemEventLoop>,
    take_space: Option<EspDefaultNvsPartition>,
}

impl NeverBeen {
    fn part_of_me(&self) -> &Peripherals {
        self.part_of_me.as_ref().unwrap()
    }
    fn to_open_eyes(self) -> Self {
        Self {
            part_of_me: Some(Peripherals::take().map_err(|e| anyhow::anyhow!("Errore periferiche: {:?}", e)).unwrap()),
            ..self
        }
    }

    fn read_through_time(self) -> Self {
        Self {
            this_time: Some(esp_idf_svc::eventloop::EspSystemEventLoop::take().unwrap()),
            ..self
        }
    }

    fn hide_hints_to_avoid_lies(self) -> Self {
        Self {
            take_space: Some(esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap()),
            ..self
        }
    }
}

fn hearing(val: bool) -> () { unsafe { esp_idf_sys::esp_wifi_set_promiscuous(val); } }
fn causing(cb: wifi_promiscuous_cb_t) -> () { unsafe { esp_idf_sys::esp_wifi_set_promiscuous_rx_cb(cb); } }
fn waiting(t: u32) -> () { esp_idf_svc::hal::delay::FreeRtos::delay_ms(t); }
fn just_wait(t: u32) -> () { waiting(t); }

mod which {
    pub mod links {
        use super::super::*;
        pub struct Inwards;
        impl Inwards {
            pub fn to(a: Channel) -> EspWifi<'static> {
                EspWifi::new(a.part_of_me.unwrap().modem, a.this_time.unwrap(), Some(a.take_space.unwrap())).unwrap()
            }
        }
    }
}

fn to_feel_a(val: &AtomicI8) -> u32 {
    let rssi = val.load(Ordering::SeqCst);
    let wait_time = if rssi <= -95 {
        3000
    } else {
        let intensity = (rssi as i32 + 95).max(0);
        let delay = 3000 - (intensity * 35);
        delay.max(200) as u32
    };
    if rssi > -128 {
        PULSE.fetch_sub(1, Ordering::SeqCst);
    }
    wait_time
}

fn to() -> () {}
trait Pulser {
    fn is_received(&mut self) -> bool;
    fn rap(&mut self) -> Result<(), EspError>;
    fn be_found(&mut self) -> &mut Self {
        self
    }
}
impl Pulser for PinDriver<'_, Gpio13, Output> {
    fn is_received(&mut self) -> bool {
        self.set_low().is_ok()
    }
    fn rap(&mut self) -> Result<(), EspError> {
        let ret = self.set_high();
        waiting(150);
        ret
    }
}
trait ToOutput {
    fn as_thin(&self) -> PinDriver<'static, Gpio13, Output>;
}
impl ToOutput for Peripherals {
    fn as_thin(&self) -> PinDriver<'static, Gpio13, Output> {
        PinDriver::output(unsafe { Gpio13::new() }).unwrap()
    }
}
// the wall
fn it_exists() -> bool {
    // only when I let you
    PULSE.load(Ordering::SeqCst) > -10
    // outside of it
}

struct Thought(PinDriver<'static, Gpio13, Output>);
struct Listening(PinDriver<'static, Gpio13, Output>);

impl From<PinDriver<'static, Gpio13, Output>> for Thought {
    fn from(p: PinDriver<'static, Gpio13, Output>) -> Self {
        Thought(p)
    }
}

impl From<Thought> for PinDriver<'static, Gpio13, Output> {
    fn from(t: Thought) -> Self {
        t.0
    }
}

impl From<PinDriver<'static, Gpio13, Output>> for Listening {
    fn from(p: PinDriver<'static, Gpio13, Output>) -> Self {
        Listening(p)
    }
}

impl From<Listening> for PinDriver<'static, Gpio13, Output> {
    fn from(t: Listening) -> Self {
        t.0
    }
}

impl Listening {
    fn the_words(self, _s: &str) -> Self {
        self
    }
}

trait Knocking {
    fn knocking(&mut self) -> Result<&mut Self, EspError>;
}
impl Knocking for Lost {
    fn knocking(&mut self) -> Result<&mut Self, EspError> { Ok(self) }
}

type Lost = PinDriver<'static, Gpio13, Output>;
type Sweet = u32;
type That = u32;
type Channel = NeverBeen;
fn you(p: PinDriver<'static, Gpio13, Output>) -> PinDriver<'static, Gpio13, Output> { p }

fn
main/*ly an error*/()
/*but*/ -> anyhow::Result<()> {
    // is the thought that keeps me going
    /* each time it */ starts(

    to()); let new
        // { part_of_me, this_time, pour_out }
            = NeverBeen::default()
          .to_open_eyes()
        .read_through_time()
      .hide_hints_to_avoid_lies();

    /* always */ let a_hidden = new.part_of_me()
        .as_thin(); // .as I can,
    to(); let _unmute = (|a: Channel| {
        which::links::Inwards
            ::to(a)}) (new); // view

    /* I feel */ unsafe {
        hearing(/* the voice
            which sounds so */true);
        causing(Some(quivers) /* to make me raw */)
    }

    let // those eyes of yours keep looking cause
    /*I'*/mut terly: Lost = Listening::from(
        you(// -
            a_hidden))// - while
        .the_words("morphing").into();

    // a one-way game which captures me
    /* in this */ loop {
        // My breath held, waiting to
        let /* this real*/mut/*most */ a_signal
        = /*is it bit*/terly.knocking()?;
        // No, but do I from
        a_signal.rap().expect("a wall,"); if it_exists() { //,
            to(); break; // down below?
        }
        while !a_signal.is_received() {
            just_wait(45);
        }; /* and */ let a_single = // feeling from
            a_signal.be_found(); // and look for
        a_single // moment to
            .set_low() /* and */ .unwrap();
        /* this */ waiting(to_feel_a(&PULSE) as Sweet as That);
    }

    Ok(())
}

static PULSE: AtomicI8 = AtomicI8::new(-128);
const VOICE: [u8; 6]   = [0x4E, 0x45, 0x41, 0x52, 0x20, 0x55];

unsafe extern "C" fn quivers(
    buf: *mut core::ffi::c_void,
    _type: esp_idf_sys::wifi_promiscuous_pkt_type_t
) {
    let pkt = buf as *mut esp_idf_sys::wifi_promiscuous_pkt_t;
    let payload = (*pkt).payload.as_ptr();
    let mac = [
        *payload.add(10),
        *payload.add(11),
        *payload.add(12),
        *payload.add(13),
        *payload.add(14),
        *payload.add(15)
    ];

    if mac == VOICE {
        let rssi = (*pkt).rx_ctrl.rssi() as i8;
        PULSE.store(rssi, Ordering::SeqCst);
    }
}
