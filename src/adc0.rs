#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Registers 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC Status and Control Registers 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC Status and Control Registers 1"]
    pub sc1c: SC1,
    #[doc = "0x0c - ADC Status and Control Registers 1"]
    pub sc1d: SC1,
    #[doc = "0x10 - ADC Configuration Register 1"]
    pub cfg1: CFG1,
    #[doc = "0x14 - ADC Configuration Register 2"]
    pub cfg2: CFG2,
    #[doc = "0x18 - ADC Data Result Register"]
    pub ra: R,
    #[doc = "0x1c - ADC Data Result Register"]
    pub rb: R,
    #[doc = "0x20 - ADC Data Result Register"]
    pub rc: R,
    #[doc = "0x24 - ADC Data Result Register"]
    pub rd: R,
    #[doc = "0x28 - Compare Value Registers"]
    pub cv1: CV,
    #[doc = "0x2c - Compare Value Registers"]
    pub cv2: CV,
    #[doc = "0x30 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x34 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x38 - ADC Offset Correction Register"]
    pub ofs: OFS,
    #[doc = "0x3c - ADC Plus-Side Gain Register"]
    pub pg: PG,
    _reserved16: [u8; 4usize],
    #[doc = "0x44 - ADC Plus-Side General Calibration Value Register"]
    pub clpd: CLPD,
    #[doc = "0x48 - ADC Plus-Side General Calibration Value Register"]
    pub clps: CLPS,
    #[doc = "0x4c - ADC Plus-Side General Calibration Value Register"]
    pub clp4: CLP4,
    #[doc = "0x50 - ADC Plus-Side General Calibration Value Register"]
    pub clp3: CLP3,
    #[doc = "0x54 - ADC Plus-Side General Calibration Value Register"]
    pub clp2: CLP2,
    #[doc = "0x58 - ADC Plus-Side General Calibration Value Register"]
    pub clp1: CLP1,
    #[doc = "0x5c - ADC Plus-Side General Calibration Value Register"]
    pub clp0: CLP0,
}
#[doc = "ADC Status and Control Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](sc1) module"]
pub type SC1 = crate::Reg<u32, _SC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC1;
#[doc = "`read()` method returns [sc1::R](sc1::R) reader structure"]
impl crate::Readable for SC1 {}
#[doc = "`write(|w| ..)` method takes [sc1::W](sc1::W) writer structure"]
impl crate::Writable for SC1 {}
#[doc = "ADC Status and Control Registers 1"]
pub mod sc1;
#[doc = "ADC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "ADC Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "ADC Data Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](r) module"]
pub type R = crate::Reg<u32, _R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R;
#[doc = "`read()` method returns [r::R](r::R) reader structure"]
impl crate::Readable for R {}
#[doc = "ADC Data Result Register"]
pub mod r;
#[doc = "Compare Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](sc2) module"]
pub type SC2 = crate::Reg<u32, _SC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC2;
#[doc = "`read()` method returns [sc2::R](sc2::R) reader structure"]
impl crate::Readable for SC2 {}
#[doc = "`write(|w| ..)` method takes [sc2::W](sc2::W) writer structure"]
impl crate::Writable for SC2 {}
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "Status and Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](sc3) module"]
pub type SC3 = crate::Reg<u32, _SC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC3;
#[doc = "`read()` method returns [sc3::R](sc3::R) reader structure"]
impl crate::Readable for SC3 {}
#[doc = "`write(|w| ..)` method takes [sc3::W](sc3::W) writer structure"]
impl crate::Writable for SC3 {}
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "ADC Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](ofs) module"]
pub type OFS = crate::Reg<u32, _OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFS;
#[doc = "`read()` method returns [ofs::R](ofs::R) reader structure"]
impl crate::Readable for OFS {}
#[doc = "`write(|w| ..)` method takes [ofs::W](ofs::W) writer structure"]
impl crate::Writable for OFS {}
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "ADC Plus-Side Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg](pg) module"]
pub type PG = crate::Reg<u32, _PG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG;
#[doc = "`read()` method returns [pg::R](pg::R) reader structure"]
impl crate::Readable for PG {}
#[doc = "`write(|w| ..)` method takes [pg::W](pg::W) writer structure"]
impl crate::Writable for PG {}
#[doc = "ADC Plus-Side Gain Register"]
pub mod pg;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpd](clpd) module"]
pub type CLPD = crate::Reg<u32, _CLPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPD;
#[doc = "`read()` method returns [clpd::R](clpd::R) reader structure"]
impl crate::Readable for CLPD {}
#[doc = "`write(|w| ..)` method takes [clpd::W](clpd::W) writer structure"]
impl crate::Writable for CLPD {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clpd;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps](clps) module"]
pub type CLPS = crate::Reg<u32, _CLPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPS;
#[doc = "`read()` method returns [clps::R](clps::R) reader structure"]
impl crate::Readable for CLPS {}
#[doc = "`write(|w| ..)` method takes [clps::W](clps::W) writer structure"]
impl crate::Writable for CLPS {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clps;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp4](clp4) module"]
pub type CLP4 = crate::Reg<u32, _CLP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP4;
#[doc = "`read()` method returns [clp4::R](clp4::R) reader structure"]
impl crate::Readable for CLP4 {}
#[doc = "`write(|w| ..)` method takes [clp4::W](clp4::W) writer structure"]
impl crate::Writable for CLP4 {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp4;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp3](clp3) module"]
pub type CLP3 = crate::Reg<u32, _CLP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP3;
#[doc = "`read()` method returns [clp3::R](clp3::R) reader structure"]
impl crate::Readable for CLP3 {}
#[doc = "`write(|w| ..)` method takes [clp3::W](clp3::W) writer structure"]
impl crate::Writable for CLP3 {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp3;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2](clp2) module"]
pub type CLP2 = crate::Reg<u32, _CLP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP2;
#[doc = "`read()` method returns [clp2::R](clp2::R) reader structure"]
impl crate::Readable for CLP2 {}
#[doc = "`write(|w| ..)` method takes [clp2::W](clp2::W) writer structure"]
impl crate::Writable for CLP2 {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp2;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1](clp1) module"]
pub type CLP1 = crate::Reg<u32, _CLP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP1;
#[doc = "`read()` method returns [clp1::R](clp1::R) reader structure"]
impl crate::Readable for CLP1 {}
#[doc = "`write(|w| ..)` method takes [clp1::W](clp1::W) writer structure"]
impl crate::Writable for CLP1 {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp1;
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp0](clp0) module"]
pub type CLP0 = crate::Reg<u32, _CLP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP0;
#[doc = "`read()` method returns [clp0::R](clp0::R) reader structure"]
impl crate::Readable for CLP0 {}
#[doc = "`write(|w| ..)` method takes [clp0::W](clp0::W) writer structure"]
impl crate::Writable for CLP0 {}
#[doc = "ADC Plus-Side General Calibration Value Register"]
pub mod clp0;
