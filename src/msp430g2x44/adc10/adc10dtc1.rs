#[doc = "Register `ADC10DTC1` reader"]
pub type R = crate::R<ADC10DTC1_SPEC>;
#[doc = "Register `ADC10DTC1` writer"]
pub type W = crate::W<ADC10DTC1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC10 Data Transfer Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10dtc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10dtc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10DTC1_SPEC;
impl crate::RegisterSpec for ADC10DTC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10dtc1::R`](R) reader structure"]
impl crate::Readable for ADC10DTC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10dtc1::W`](W) writer structure"]
impl crate::Writable for ADC10DTC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10DTC1 to value 0"]
impl crate::Resettable for ADC10DTC1_SPEC {
    const RESET_VALUE: u8 = 0;
}
