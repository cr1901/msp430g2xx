#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3ren: P3ren,
    _reserved1: [u8; 0x07],
    p3in: P3in,
    p3out: P3out,
    p3dir: P3dir,
    p3sel: P3sel,
    _reserved5: [u8; 0x27],
    p3sel2: P3sel2,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3ren {
        &self.p3ren
    }
    #[doc = "0x08 - Port 3 Input"]
    #[inline(always)]
    pub const fn p3in(&self) -> &P3in {
        &self.p3in
    }
    #[doc = "0x09 - Port 3 Output"]
    #[inline(always)]
    pub const fn p3out(&self) -> &P3out {
        &self.p3out
    }
    #[doc = "0x0a - Port 3 Direction"]
    #[inline(always)]
    pub const fn p3dir(&self) -> &P3dir {
        &self.p3dir
    }
    #[doc = "0x0b - Port 3 Selection"]
    #[inline(always)]
    pub const fn p3sel(&self) -> &P3sel {
        &self.p3sel
    }
    #[doc = "0x33 - Port 3 Selection 2"]
    #[inline(always)]
    pub const fn p3sel2(&self) -> &P3sel2 {
        &self.p3sel2
    }
}
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`]
module"]
#[doc(alias = "P3REN")]
pub type P3ren = crate::Reg<p3ren::P3renSpec>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P3IN (rw) register accessor: Port 3 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p3in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3in`]
module"]
#[doc(alias = "P3IN")]
pub type P3in = crate::Reg<p3in::P3inSpec>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P3OUT (rw) register accessor: Port 3 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p3out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3out`]
module"]
#[doc(alias = "P3OUT")]
pub type P3out = crate::Reg<p3out::P3outSpec>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P3DIR (rw) register accessor: Port 3 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p3dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3dir`]
module"]
#[doc(alias = "P3DIR")]
pub type P3dir = crate::Reg<p3dir::P3dirSpec>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P3SEL (rw) register accessor: Port 3 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel`]
module"]
#[doc(alias = "P3SEL")]
pub type P3sel = crate::Reg<p3sel::P3selSpec>;
#[doc = "Port 3 Selection"]
pub mod p3sel;
#[doc = "P3SEL2 (rw) register accessor: Port 3 Selection 2\n\nYou can [`read`](crate::Reg::read) this register and get [`p3sel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3sel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3sel2`]
module"]
#[doc(alias = "P3SEL2")]
pub type P3sel2 = crate::Reg<p3sel2::P3sel2Spec>;
#[doc = "Port 3 Selection 2"]
pub mod p3sel2;
