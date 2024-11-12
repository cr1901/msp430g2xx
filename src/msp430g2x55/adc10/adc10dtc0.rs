#[doc = "Register `ADC10DTC0` reader"]
pub type R = crate::R<Adc10dtc0Spec>;
#[doc = "Register `ADC10DTC0` writer"]
pub type W = crate::W<Adc10dtc0Spec>;
#[doc = "Field `ADC10FETCH` reader - This bit should normally be reset"]
pub type Adc10fetchR = crate::BitReader;
#[doc = "Field `ADC10FETCH` writer - This bit should normally be reset"]
pub type Adc10fetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10B1` reader - ADC10 block one"]
pub type Adc10b1R = crate::BitReader;
#[doc = "Field `ADC10B1` writer - ADC10 block one"]
pub type Adc10b1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10CT` reader - ADC10 continuous transfer"]
pub type Adc10ctR = crate::BitReader;
#[doc = "Field `ADC10CT` writer - ADC10 continuous transfer"]
pub type Adc10ctW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10TB` reader - ADC10 two-block mode"]
pub type Adc10tbR = crate::BitReader;
#[doc = "Field `ADC10TB` writer - ADC10 two-block mode"]
pub type Adc10tbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&self) -> Adc10fetchR {
        Adc10fetchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&self) -> Adc10b1R {
        Adc10b1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&self) -> Adc10ctR {
        Adc10ctR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&self) -> Adc10tbR {
        Adc10tbR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit should normally be reset"]
    #[inline(always)]
    pub fn adc10fetch(&mut self) -> Adc10fetchW<Adc10dtc0Spec> {
        Adc10fetchW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC10 block one"]
    #[inline(always)]
    pub fn adc10b1(&mut self) -> Adc10b1W<Adc10dtc0Spec> {
        Adc10b1W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC10 continuous transfer"]
    #[inline(always)]
    pub fn adc10ct(&mut self) -> Adc10ctW<Adc10dtc0Spec> {
        Adc10ctW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC10 two-block mode"]
    #[inline(always)]
    pub fn adc10tb(&mut self) -> Adc10tbW<Adc10dtc0Spec> {
        Adc10tbW::new(self, 3)
    }
}
#[doc = "ADC10 Data Transfer Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10dtc0Spec;
impl crate::RegisterSpec for Adc10dtc0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10dtc0::R`](R) reader structure"]
impl crate::Readable for Adc10dtc0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10dtc0::W`](W) writer structure"]
impl crate::Writable for Adc10dtc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10DTC0 to value 0"]
impl crate::Resettable for Adc10dtc0Spec {
    const RESET_VALUE: u8 = 0;
}
