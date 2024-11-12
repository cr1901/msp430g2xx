#[doc = "Register `ADC10AE1` reader"]
pub type R = crate::R<Adc10ae1Spec>;
#[doc = "Register `ADC10AE1` writer"]
pub type W = crate::W<Adc10ae1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC10 Analog Enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10ae1Spec;
impl crate::RegisterSpec for Adc10ae1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10ae1::R`](R) reader structure"]
impl crate::Readable for Adc10ae1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10ae1::W`](W) writer structure"]
impl crate::Writable for Adc10ae1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10AE1 to value 0"]
impl crate::Resettable for Adc10ae1Spec {
    const RESET_VALUE: u8 = 0;
}
