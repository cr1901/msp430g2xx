#[doc = "Register `ADC10MEM` reader"]
pub type R = crate::R<ADC10MEM_SPEC>;
#[doc = "Register `ADC10MEM` writer"]
pub type W = crate::W<ADC10MEM_SPEC>;
#[doc = "Field `ADC10MEM` reader - ADC10 Memory register"]
pub type ADC10MEM_R = crate::FieldReader<u16>;
#[doc = "Field `ADC10MEM` writer - ADC10 Memory register"]
pub type ADC10MEM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> ADC10MEM_R {
        ADC10MEM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&mut self) -> ADC10MEM_W<ADC10MEM_SPEC> {
        ADC10MEM_W::new(self, 0)
    }
}
#[doc = "ADC10 Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC10MEM_SPEC;
impl crate::RegisterSpec for ADC10MEM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10mem::R`](R) reader structure"]
impl crate::Readable for ADC10MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc10mem::W`](W) writer structure"]
impl crate::Writable for ADC10MEM_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC10MEM to value 0"]
impl crate::Resettable for ADC10MEM_SPEC {
    const RESET_VALUE: u16 = 0;
}
