#[doc = "Register `ADC10MEM` reader"]
pub type R = crate::R<Adc10memSpec>;
#[doc = "Register `ADC10MEM` writer"]
pub type W = crate::W<Adc10memSpec>;
#[doc = "Field `ADC10MEM` reader - ADC10 Memory register"]
pub type Adc10memR = crate::FieldReader<u16>;
#[doc = "Field `ADC10MEM` writer - ADC10 Memory register"]
pub type Adc10memW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&self) -> Adc10memR {
        Adc10memR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Memory register"]
    #[inline(always)]
    pub fn adc10mem(&mut self) -> Adc10memW<Adc10memSpec> {
        Adc10memW::new(self, 0)
    }
}
#[doc = "ADC10 Memory\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10memSpec;
impl crate::RegisterSpec for Adc10memSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10mem::R`](R) reader structure"]
impl crate::Readable for Adc10memSpec {}
#[doc = "`write(|w| ..)` method takes [`adc10mem::W`](W) writer structure"]
impl crate::Writable for Adc10memSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC10MEM to value 0"]
impl crate::Resettable for Adc10memSpec {
    const RESET_VALUE: u16 = 0;
}
