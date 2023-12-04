
//! - START HDR HDR
//！- HDR_RCVD HDR OPEN
//！- HDR_SENT OPEN HDR
//！- HDR_EXCH OPEN OPEN
//！- OPEN_RCVD OPEN *
//！- OPEN_SENT ** OPEN
//！- OPEN_PIPE ** HDR
//！- CLOSE_PIPE - OPEN TCP Close for Write
//！- OC_PIPE - HDR TCP Close for Write
//！- OPENED * *
//！- CLOSE_RCVD * - TCP Close for Read
//！- CLOSE_SENT - * TCP Close for Write
//！- DISCARDING - * TCP Close for Write
//！- END - - TCP Close


pub struct Start {

}



pub struct HdrRcvd {

}
pub struct HdrSent {

}
pub struct HdrExch {

}
pub struct OpenRcvd {

}
pub struct OpenSent {

}
pub struct OpenPipe {

}
pub struct ClosePipe {

}
pub struct OcPipe {

}
pub struct Opened {

}
pub struct CloseRcvd {

}
pub struct CloseSent {

}
pub struct Discarding {

}
pub struct End {

}