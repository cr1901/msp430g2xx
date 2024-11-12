#[doc = "Register `BCSCTL1` reader"]
pub type R = crate::R<Bcsctl1Spec>;
#[doc = "Register `BCSCTL1` writer"]
pub type W = crate::W<Bcsctl1Spec>;
#[doc = "Field `BCSCTL1` reader - Basic Clock System Control 1 register"]
pub type Bcsctl1R = crate::FieldReader;
#[doc = "Field `BCSCTL1` writer - Basic Clock System Control 1 register"]
pub type Bcsctl1W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
#[doc = "Field `RSEL` reader - Range Select Bit 0"]
pub type RselR = crate::FieldReader;
#[doc = "Field `RSEL` writer - Range Select Bit 0"]
pub type RselW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "ACLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Diva {
    #[doc = "0: ACLK Divider 0: /1"]
    Diva0 = 0,
    #[doc = "1: ACLK Divider 1: /2"]
    Diva1 = 1,
    #[doc = "2: ACLK Divider 2: /4"]
    Diva2 = 2,
    #[doc = "3: ACLK Divider 3: /8"]
    Diva3 = 3,
}
impl From<Diva> for u8 {
    #[inline(always)]
    fn from(variant: Diva) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Diva {
    type Ux = u8;
}
impl crate::IsEnum for Diva {}
#[doc = "Field `DIVA` reader - ACLK Divider 0"]
pub type DivaR = crate::FieldReader<Diva>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diva {
        match self.bits {
            0 => Diva::Diva0,
            1 => Diva::Diva1,
            2 => Diva::Diva2,
            3 => Diva::Diva3,
            _ => unreachable!(),
        }
    }
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == Diva::Diva0
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == Diva::Diva1
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == Diva::Diva2
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == Diva::Diva3
    }
}
#[doc = "Field `DIVA` writer - ACLK Divider 0"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Diva, crate::Safe>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva0)
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva1)
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva2)
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut crate::W<REG> {
        self.variant(Diva::Diva3)
    }
}
#[doc = "Field `XTS` reader - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XtsR = crate::BitReader;
#[doc = "Field `XTS` writer - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XT2OFF` reader - Enable XT2CLK"]
pub type Xt2offR = crate::BitReader;
#[doc = "Field `XT2OFF` writer - Enable XT2CLK"]
pub type Xt2offW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Basic Clock System Control 1 register"]
    #[inline(always)]
    pub fn bcsctl1(&self) -> Bcsctl1R {
        Bcsctl1R::new(self.bits)
    }
    #[doc = "Bits 0:3 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel(&self) -> RselR {
        RselR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&self) -> XtsR {
        XtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&self) -> Xt2offR {
        Xt2offR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Basic Clock System Control 1 register"]
    #[inline(always)]
    pub fn bcsctl1(&mut self) -> Bcsctl1W<Bcsctl1Spec> {
        Bcsctl1W::new(self, 0)
    }
    #[doc = "Bits 0:3 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel(&mut self) -> RselW<Bcsctl1Spec> {
        RselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<Bcsctl1Spec> {
        DivaW::new(self, 4)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&mut self) -> XtsW<Bcsctl1Spec> {
        XtsW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&mut self) -> Xt2offW<Bcsctl1Spec> {
        Xt2offW::new(self, 7)
    }
}
#[doc = "Basic Clock System Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bcsctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcsctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bcsctl1Spec;
impl crate::RegisterSpec for Bcsctl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcsctl1::R`](R) reader structure"]
impl crate::Readable for Bcsctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcsctl1::W`](W) writer structure"]
impl crate::Writable for Bcsctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BCSCTL1 to value 0"]
impl crate::Resettable for Bcsctl1Spec {
    const RESET_VALUE: u8 = 0;
}
