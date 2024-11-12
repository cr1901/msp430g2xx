#[doc = "Register `USICKCTL` reader"]
pub type R = crate::R<UsickctlSpec>;
#[doc = "Register `USICKCTL` writer"]
pub type W = crate::W<UsickctlSpec>;
#[doc = "Field `USISWCLK` reader - USI Software Clock"]
pub type UsiswclkR = crate::BitReader;
#[doc = "Field `USISWCLK` writer - USI Software Clock"]
pub type UsiswclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USICKPL` reader - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
pub type UsickplR = crate::BitReader;
#[doc = "Field `USICKPL` writer - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
pub type UsickplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USI Clock Source Select 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usissel {
    #[doc = "0: USI Clock Source: 0"]
    Usissel0 = 0,
    #[doc = "1: USI Clock Source: 1"]
    Usissel1 = 1,
    #[doc = "2: USI Clock Source: 2"]
    Usissel2 = 2,
    #[doc = "3: USI Clock Source: 3"]
    Usissel3 = 3,
    #[doc = "4: USI Clock Source: 4"]
    Usissel4 = 4,
    #[doc = "5: USI Clock Source: 5"]
    Usissel5 = 5,
    #[doc = "6: USI Clock Source: 6"]
    Usissel6 = 6,
    #[doc = "7: USI Clock Source: 7"]
    Usissel7 = 7,
}
impl From<Usissel> for u8 {
    #[inline(always)]
    fn from(variant: Usissel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usissel {
    type Ux = u8;
}
impl crate::IsEnum for Usissel {}
#[doc = "Field `USISSEL` reader - USI Clock Source Select 2"]
pub type UsisselR = crate::FieldReader<Usissel>;
impl UsisselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usissel {
        match self.bits {
            0 => Usissel::Usissel0,
            1 => Usissel::Usissel1,
            2 => Usissel::Usissel2,
            3 => Usissel::Usissel3,
            4 => Usissel::Usissel4,
            5 => Usissel::Usissel5,
            6 => Usissel::Usissel6,
            7 => Usissel::Usissel7,
            _ => unreachable!(),
        }
    }
    #[doc = "USI Clock Source: 0"]
    #[inline(always)]
    pub fn is_usissel_0(&self) -> bool {
        *self == Usissel::Usissel0
    }
    #[doc = "USI Clock Source: 1"]
    #[inline(always)]
    pub fn is_usissel_1(&self) -> bool {
        *self == Usissel::Usissel1
    }
    #[doc = "USI Clock Source: 2"]
    #[inline(always)]
    pub fn is_usissel_2(&self) -> bool {
        *self == Usissel::Usissel2
    }
    #[doc = "USI Clock Source: 3"]
    #[inline(always)]
    pub fn is_usissel_3(&self) -> bool {
        *self == Usissel::Usissel3
    }
    #[doc = "USI Clock Source: 4"]
    #[inline(always)]
    pub fn is_usissel_4(&self) -> bool {
        *self == Usissel::Usissel4
    }
    #[doc = "USI Clock Source: 5"]
    #[inline(always)]
    pub fn is_usissel_5(&self) -> bool {
        *self == Usissel::Usissel5
    }
    #[doc = "USI Clock Source: 6"]
    #[inline(always)]
    pub fn is_usissel_6(&self) -> bool {
        *self == Usissel::Usissel6
    }
    #[doc = "USI Clock Source: 7"]
    #[inline(always)]
    pub fn is_usissel_7(&self) -> bool {
        *self == Usissel::Usissel7
    }
}
#[doc = "Field `USISSEL` writer - USI Clock Source Select 2"]
pub type UsisselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usissel, crate::Safe>;
impl<'a, REG> UsisselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USI Clock Source: 0"]
    #[inline(always)]
    pub fn usissel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel0)
    }
    #[doc = "USI Clock Source: 1"]
    #[inline(always)]
    pub fn usissel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel1)
    }
    #[doc = "USI Clock Source: 2"]
    #[inline(always)]
    pub fn usissel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel2)
    }
    #[doc = "USI Clock Source: 3"]
    #[inline(always)]
    pub fn usissel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel3)
    }
    #[doc = "USI Clock Source: 4"]
    #[inline(always)]
    pub fn usissel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel4)
    }
    #[doc = "USI Clock Source: 5"]
    #[inline(always)]
    pub fn usissel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel5)
    }
    #[doc = "USI Clock Source: 6"]
    #[inline(always)]
    pub fn usissel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel6)
    }
    #[doc = "USI Clock Source: 7"]
    #[inline(always)]
    pub fn usissel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Usissel::Usissel7)
    }
}
#[doc = "USI Clock Divider 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usidiv {
    #[doc = "0: USI Clock Divider: 0"]
    Usidiv0 = 0,
    #[doc = "1: USI Clock Divider: 1"]
    Usidiv1 = 1,
    #[doc = "2: USI Clock Divider: 2"]
    Usidiv2 = 2,
    #[doc = "3: USI Clock Divider: 3"]
    Usidiv3 = 3,
    #[doc = "4: USI Clock Divider: 4"]
    Usidiv4 = 4,
    #[doc = "5: USI Clock Divider: 5"]
    Usidiv5 = 5,
    #[doc = "6: USI Clock Divider: 6"]
    Usidiv6 = 6,
    #[doc = "7: USI Clock Divider: 7"]
    Usidiv7 = 7,
}
impl From<Usidiv> for u8 {
    #[inline(always)]
    fn from(variant: Usidiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usidiv {
    type Ux = u8;
}
impl crate::IsEnum for Usidiv {}
#[doc = "Field `USIDIV` reader - USI Clock Divider 2"]
pub type UsidivR = crate::FieldReader<Usidiv>;
impl UsidivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usidiv {
        match self.bits {
            0 => Usidiv::Usidiv0,
            1 => Usidiv::Usidiv1,
            2 => Usidiv::Usidiv2,
            3 => Usidiv::Usidiv3,
            4 => Usidiv::Usidiv4,
            5 => Usidiv::Usidiv5,
            6 => Usidiv::Usidiv6,
            7 => Usidiv::Usidiv7,
            _ => unreachable!(),
        }
    }
    #[doc = "USI Clock Divider: 0"]
    #[inline(always)]
    pub fn is_usidiv_0(&self) -> bool {
        *self == Usidiv::Usidiv0
    }
    #[doc = "USI Clock Divider: 1"]
    #[inline(always)]
    pub fn is_usidiv_1(&self) -> bool {
        *self == Usidiv::Usidiv1
    }
    #[doc = "USI Clock Divider: 2"]
    #[inline(always)]
    pub fn is_usidiv_2(&self) -> bool {
        *self == Usidiv::Usidiv2
    }
    #[doc = "USI Clock Divider: 3"]
    #[inline(always)]
    pub fn is_usidiv_3(&self) -> bool {
        *self == Usidiv::Usidiv3
    }
    #[doc = "USI Clock Divider: 4"]
    #[inline(always)]
    pub fn is_usidiv_4(&self) -> bool {
        *self == Usidiv::Usidiv4
    }
    #[doc = "USI Clock Divider: 5"]
    #[inline(always)]
    pub fn is_usidiv_5(&self) -> bool {
        *self == Usidiv::Usidiv5
    }
    #[doc = "USI Clock Divider: 6"]
    #[inline(always)]
    pub fn is_usidiv_6(&self) -> bool {
        *self == Usidiv::Usidiv6
    }
    #[doc = "USI Clock Divider: 7"]
    #[inline(always)]
    pub fn is_usidiv_7(&self) -> bool {
        *self == Usidiv::Usidiv7
    }
}
#[doc = "Field `USIDIV` writer - USI Clock Divider 2"]
pub type UsidivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usidiv, crate::Safe>;
impl<'a, REG> UsidivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USI Clock Divider: 0"]
    #[inline(always)]
    pub fn usidiv_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv0)
    }
    #[doc = "USI Clock Divider: 1"]
    #[inline(always)]
    pub fn usidiv_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv1)
    }
    #[doc = "USI Clock Divider: 2"]
    #[inline(always)]
    pub fn usidiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv2)
    }
    #[doc = "USI Clock Divider: 3"]
    #[inline(always)]
    pub fn usidiv_3(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv3)
    }
    #[doc = "USI Clock Divider: 4"]
    #[inline(always)]
    pub fn usidiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv4)
    }
    #[doc = "USI Clock Divider: 5"]
    #[inline(always)]
    pub fn usidiv_5(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv5)
    }
    #[doc = "USI Clock Divider: 6"]
    #[inline(always)]
    pub fn usidiv_6(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv6)
    }
    #[doc = "USI Clock Divider: 7"]
    #[inline(always)]
    pub fn usidiv_7(self) -> &'a mut crate::W<REG> {
        self.variant(Usidiv::Usidiv7)
    }
}
impl R {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    pub fn usiswclk(&self) -> UsiswclkR {
        UsiswclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    pub fn usickpl(&self) -> UsickplR {
        UsickplR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    pub fn usissel(&self) -> UsisselR {
        UsisselR::new((self.bits >> 2) & 7)
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    pub fn usidiv(&self) -> UsidivR {
        UsidivR::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - USI Software Clock"]
    #[inline(always)]
    pub fn usiswclk(&mut self) -> UsiswclkW<UsickctlSpec> {
        UsiswclkW::new(self, 0)
    }
    #[doc = "Bit 1 - USI Clock Polarity 0:Inactive=Low / 1:Inactive=High"]
    #[inline(always)]
    pub fn usickpl(&mut self) -> UsickplW<UsickctlSpec> {
        UsickplW::new(self, 1)
    }
    #[doc = "Bits 2:4 - USI Clock Source Select 2"]
    #[inline(always)]
    pub fn usissel(&mut self) -> UsisselW<UsickctlSpec> {
        UsisselW::new(self, 2)
    }
    #[doc = "Bits 5:7 - USI Clock Divider 2"]
    #[inline(always)]
    pub fn usidiv(&mut self) -> UsidivW<UsickctlSpec> {
        UsidivW::new(self, 5)
    }
}
#[doc = "USI Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usickctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usickctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsickctlSpec;
impl crate::RegisterSpec for UsickctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usickctl::R`](R) reader structure"]
impl crate::Readable for UsickctlSpec {}
#[doc = "`write(|w| ..)` method takes [`usickctl::W`](W) writer structure"]
impl crate::Writable for UsickctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USICKCTL to value 0"]
impl crate::Resettable for UsickctlSpec {
    const RESET_VALUE: u8 = 0;
}
