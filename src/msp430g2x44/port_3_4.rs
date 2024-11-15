#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    p3ren: P3REN,
    p4ren: P4REN,
    _reserved2: [u8; 0x06],
    p3in: P3IN,
    p3out: P3OUT,
    p3dir: P3DIR,
    p3sel: P3SEL,
    p4in: P4IN,
    p4out: P4OUT,
    p4dir: P4DIR,
    p4sel: P4SEL,
}
impl RegisterBlock {
    #[doc = "0x00 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub const fn p3ren(&self) -> &P3REN {
        &self.p3ren
    }
    #[doc = "0x01 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub const fn p4ren(&self) -> &P4REN {
        &self.p4ren
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
    #[doc = "0x0c - Port 4 Input"]
    #[inline(always)]
    pub const fn p4in(&self) -> &P4IN {
        &self.p4in
    }
    #[doc = "0x0d - Port 4 Output"]
    #[inline(always)]
    pub const fn p4out(&self) -> &P4OUT {
        &self.p4out
    }
    #[doc = "0x0e - Port 4 Direction"]
    #[inline(always)]
    pub const fn p4dir(&self) -> &P4DIR {
        &self.p4dir
    }
    #[doc = "0x0f - Port 4 Selection"]
    #[inline(always)]
    pub const fn p4sel(&self) -> &P4SEL {
        &self.p4sel
    }
}
#[doc = "P3REN (rw) register accessor: Port 3 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p3ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p3ren`]
module"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P4REN (rw) register accessor: Port 4 Resistor Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p4ren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4ren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4ren`]
module"]
pub type P4REN = crate::Reg<p4ren::P4REN_SPEC>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
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
#[doc = "P4IN (rw) register accessor: Port 4 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p4in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4in`]
module"]
pub type P4IN = crate::Reg<p4in::P4IN_SPEC>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P4OUT (rw) register accessor: Port 4 Output\n\nYou can [`read`](crate::Reg::read) this register and get [`p4out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4out`]
module"]
pub type P4OUT = crate::Reg<p4out::P4OUT_SPEC>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P4DIR (rw) register accessor: Port 4 Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`p4dir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4dir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4dir`]
module"]
pub type P4DIR = crate::Reg<p4dir::P4DIR_SPEC>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P4SEL (rw) register accessor: Port 4 Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`p4sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@p4sel`]
module"]
pub type P4SEL = crate::Reg<p4sel::P4SEL_SPEC>;
#[doc = "Port 4 Selection"]
pub mod p4sel;
