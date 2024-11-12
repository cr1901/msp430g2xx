#[doc = "Register `ADC10SA` reader"]
pub type R = crate::R<Adc10saSpec>;
#[doc = "Register `ADC10SA` writer"]
pub type W = crate::W<Adc10saSpec>;
#[doc = "Field `ADC10SA` reader - ADC10 Data Transfer Start Address register"]
pub type Adc10saR = crate::FieldReader<u16>;
#[doc = "Field `ADC10SA` writer - ADC10 Data Transfer Start Address register"]
pub type Adc10saW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&self) -> Adc10saR {
        Adc10saR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC10 Data Transfer Start Address register"]
    #[inline(always)]
    pub fn adc10sa(&mut self) -> Adc10saW<Adc10saSpec> {
        Adc10saW::new(self, 0)
    }
}
#[doc = "ADC10 Data Transfer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10saSpec;
impl crate::RegisterSpec for Adc10saSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10sa::R`](R) reader structure"]
impl crate::Readable for Adc10saSpec {}
#[doc = "`write(|w| ..)` method takes [`adc10sa::W`](W) writer structure"]
impl crate::Writable for Adc10saSpec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC10SA to value 0"]
impl crate::Resettable for Adc10saSpec {
    const RESET_VALUE: u16 = 0;
}
