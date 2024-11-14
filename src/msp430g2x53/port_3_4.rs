#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3ren: P3REN,
    _reserved1: [u8; 0x07],
    p3in: P3IN,
    p3out: P3OUT,
    p3dir: P3DIR,
    p3sel: P3SEL,
    _reserved5: [u8; 0x27],
    p3sel2: P3SEL2,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3REN {
        &self.p3ren
    }
    #[doc = "0x08 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3IN {
        &self.p3in
    }
    #[doc = "0x09 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3OUT {
        &self.p3out
    }
    #[doc = "0x0a - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3DIR {
        &self.p3dir
    }
    #[doc = "0x0b - Port 3 Selection"]
    #[inline(always)]
    pub const fn p3sel(&self) -> &P3SEL {
        &self.p3sel
    }
    #[doc = "0x33 - Port 3 Selection 2"]
    #[inline(always)]
    pub const fn p3sel2(&self) -> &P3SEL2 {
        &self.p3sel2
    }
}
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`]
module"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`]
module"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`]
module"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`]
module"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL (rw) register accessor: Port 3 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel`]
module"]
pub type P3SEL = crate::Reg<p3sel::P3SEL_SPEC>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P3SEL2 (rw) register accessor: Port 3 Selection 2\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel2`]
module"]
pub type P3SEL2 = crate::Reg<p3sel2::P3SEL2_SPEC>;
#[doc = "Port 3 Selection 2"]
pub mod p3sel2;
