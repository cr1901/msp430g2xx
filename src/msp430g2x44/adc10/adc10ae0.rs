#[doc = "Register `ADC10AE0` reader"]
pub type R = crate::R<Adc10ae0Spec>;
#[doc = "Register `ADC10AE0` writer"]
pub type W = crate::W<Adc10ae0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ADC10 Analog Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10ae0Spec;
impl crate::RegisterSpec for Adc10ae0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10ae0::R`](R) reader structure"]
impl crate::Readable for Adc10ae0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10ae0::W`](W) writer structure"]
impl crate::Writable for Adc10ae0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10AE0 to value 0"]
impl crate::Resettable for Adc10ae0Spec {
    const RESET_VALUE: u8 = 0;
}