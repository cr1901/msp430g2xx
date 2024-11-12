#[doc = "Register `ADC10DTC1` reader"]
pub type R = crate::R<Adc10dtc1Spec>;
#[doc = "Register `ADC10DTC1` writer"]
pub type W = crate::W<Adc10dtc1Spec>;
#[doc = "Field `ADC10DTC1` reader - ADC10 Data Transfer Control 1 register"]
pub type Adc10dtc1R = crate::FieldReader;
#[doc = "Field `ADC10DTC1` writer - ADC10 Data Transfer Control 1 register"]
pub type Adc10dtc1W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&self) -> Adc10dtc1R {
        Adc10dtc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC10 Data Transfer Control 1 register"]
    #[inline(always)]
    pub fn adc10dtc1(&mut self) -> Adc10dtc1W<Adc10dtc1Spec> {
        Adc10dtc1W::new(self, 0)
    }
}
#[doc = "ADC10 Data Transfer Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10dtc1Spec;
impl crate::RegisterSpec for Adc10dtc1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10dtc1::R`](R) reader structure"]
impl crate::Readable for Adc10dtc1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10dtc1::W`](W) writer structure"]
impl crate::Writable for Adc10dtc1Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10DTC1 to value 0"]
impl crate::Resettable for Adc10dtc1Spec {
    const RESET_VALUE: u8 = 0;
}
