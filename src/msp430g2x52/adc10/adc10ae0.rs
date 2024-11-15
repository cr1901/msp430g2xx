#[doc = "Register `ADC10AE0` reader"]
pub type R = crate::R<ADC10AE0_SPEC>;
#[doc = "Register `ADC10AE0` writer"]
pub type W = crate::W<ADC10AE0_SPEC>;
#[doc = "Field `ADC10AE0` reader - ADC10 Analog Enable 0 register"]
pub type ADC10AE0_R = crate::FieldReader;
#[doc = "Field `ADC10AE0` writer - ADC10 Analog Enable 0 register"]
pub type ADC10AE0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:7 - ADC10 Analog Enable 0 register"]
    #[inline(always)]
    pub fn adc10ae0(&self) -> ADC10AE0_R {
        ADC10AE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC10 Analog Enable 0 register"]
    #[inline(always)]
    pub fn adc10ae0(&mut self) -> ADC10AE0_W<ADC10AE0_SPEC> {
        ADC10AE0_W::new(self, 0)
    }
}
#[doc = "ADC10 Analog Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ae0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ae0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10AE0_SPEC;
impl crate::RegisterSpec for ADC10AE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adc10ae0::R`](R) reader structure"]
impl crate::Readable for ADC10AE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10ae0::W`](W) writer structure"]
impl crate::Writable for ADC10AE0_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADC10AE0 to value 0"]
impl crate::Resettable for ADC10AE0_SPEC {
    const RESET_VALUE: u8 = 0;
}
