#[doc = "Register `ADC10CTL0` reader"]
pub type R = crate::R<Adc10ctl0Spec>;
#[doc = "Register `ADC10CTL0` writer"]
pub type W = crate::W<Adc10ctl0Spec>;
#[doc = "Field `ADC10SC` reader - ADC10 Start Conversion"]
pub type Adc10scR = crate::BitReader;
#[doc = "Field `ADC10SC` writer - ADC10 Start Conversion"]
pub type Adc10scW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENC` reader - ADC10 Enable Conversion"]
pub type EncR = crate::BitReader;
#[doc = "Field `ENC` writer - ADC10 Enable Conversion"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10IFG` reader - ADC10 Interrupt Flag"]
pub type Adc10ifgR = crate::BitReader;
#[doc = "Field `ADC10IFG` writer - ADC10 Interrupt Flag"]
pub type Adc10ifgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10IE` reader - ADC10 Interrupt Enalbe"]
pub type Adc10ieR = crate::BitReader;
#[doc = "Field `ADC10IE` writer - ADC10 Interrupt Enalbe"]
pub type Adc10ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10ON` reader - ADC10 On/Enable"]
pub type Adc10onR = crate::BitReader;
#[doc = "Field `ADC10ON` writer - ADC10 On/Enable"]
pub type Adc10onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFON` reader - ADC10 Reference on"]
pub type RefonR = crate::BitReader;
#[doc = "Field `REFON` writer - ADC10 Reference on"]
pub type RefonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF2_5V` reader - ADC10 Ref 0:1.5V / 1:2.5V"]
pub type Ref2_5vR = crate::BitReader;
#[doc = "Field `REF2_5V` writer - ADC10 Ref 0:1.5V / 1:2.5V"]
pub type Ref2_5vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - ADC10 Multiple SampleConversion"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - ADC10 Multiple SampleConversion"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBURST` reader - ADC10 Reference Burst Mode"]
pub type RefburstR = crate::BitReader;
#[doc = "Field `REFBURST` writer - ADC10 Reference Burst Mode"]
pub type RefburstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFOUT` reader - ADC10 Enalbe output of Ref."]
pub type RefoutR = crate::BitReader;
#[doc = "Field `REFOUT` writer - ADC10 Enalbe output of Ref."]
pub type RefoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10SR` reader - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub type Adc10srR = crate::BitReader;
#[doc = "Field `ADC10SR` writer - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
pub type Adc10srW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC10 Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc10sht {
    #[doc = "0: 4 x ADC10CLKs"]
    Adc10sht0 = 0,
    #[doc = "1: 8 x ADC10CLKs"]
    Adc10sht1 = 1,
    #[doc = "2: 16 x ADC10CLKs"]
    Adc10sht2 = 2,
    #[doc = "3: 64 x ADC10CLKs"]
    Adc10sht3 = 3,
}
impl From<Adc10sht> for u8 {
    #[inline(always)]
    fn from(variant: Adc10sht) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc10sht {
    type Ux = u8;
}
impl crate::IsEnum for Adc10sht {}
#[doc = "Field `ADC10SHT` reader - ADC10 Sample Hold Select Bit: 0"]
pub type Adc10shtR = crate::FieldReader<Adc10sht>;
impl Adc10shtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc10sht {
        match self.bits {
            0 => Adc10sht::Adc10sht0,
            1 => Adc10sht::Adc10sht1,
            2 => Adc10sht::Adc10sht2,
            3 => Adc10sht::Adc10sht3,
            _ => unreachable!(),
        }
    }
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_0(&self) -> bool {
        *self == Adc10sht::Adc10sht0
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_1(&self) -> bool {
        *self == Adc10sht::Adc10sht1
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_2(&self) -> bool {
        *self == Adc10sht::Adc10sht2
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn is_adc10sht_3(&self) -> bool {
        *self == Adc10sht::Adc10sht3
    }
}
#[doc = "Field `ADC10SHT` writer - ADC10 Sample Hold Select Bit: 0"]
pub type Adc10shtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc10sht, crate::Safe>;
impl<'a, REG> Adc10shtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10sht::Adc10sht0)
    }
    #[doc = "8 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10sht::Adc10sht1)
    }
    #[doc = "16 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10sht::Adc10sht2)
    }
    #[doc = "64 x ADC10CLKs"]
    #[inline(always)]
    pub fn adc10sht_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10sht::Adc10sht3)
    }
}
#[doc = "ADC10 Reference Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sref {
    #[doc = "0: VR+ = AVCC and VR- = AVSS"]
    Sref0 = 0,
    #[doc = "1: VR+ = VREF+ and VR- = AVSS"]
    Sref1 = 1,
    #[doc = "2: VR+ = VEREF+ and VR- = AVSS"]
    Sref2 = 2,
    #[doc = "3: VR+ = VEREF+ and VR- = AVSS"]
    Sref3 = 3,
    #[doc = "4: VR+ = AVCC and VR- = VREF-/VEREF-"]
    Sref4 = 4,
    #[doc = "5: VR+ = VREF+ and VR- = VREF-/VEREF-"]
    Sref5 = 5,
    #[doc = "6: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    Sref6 = 6,
    #[doc = "7: VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    Sref7 = 7,
}
impl From<Sref> for u8 {
    #[inline(always)]
    fn from(variant: Sref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sref {
    type Ux = u8;
}
impl crate::IsEnum for Sref {}
#[doc = "Field `SREF` reader - ADC10 Reference Select Bit: 0"]
pub type SrefR = crate::FieldReader<Sref>;
impl SrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sref {
        match self.bits {
            0 => Sref::Sref0,
            1 => Sref::Sref1,
            2 => Sref::Sref2,
            3 => Sref::Sref3,
            4 => Sref::Sref4,
            5 => Sref::Sref5,
            6 => Sref::Sref6,
            7 => Sref::Sref7,
            _ => unreachable!(),
        }
    }
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_0(&self) -> bool {
        *self == Sref::Sref0
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_1(&self) -> bool {
        *self == Sref::Sref1
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_2(&self) -> bool {
        *self == Sref::Sref2
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn is_sref_3(&self) -> bool {
        *self == Sref::Sref3
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_4(&self) -> bool {
        *self == Sref::Sref4
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_5(&self) -> bool {
        *self == Sref::Sref5
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_6(&self) -> bool {
        *self == Sref::Sref6
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn is_sref_7(&self) -> bool {
        *self == Sref::Sref7
    }
}
#[doc = "Field `SREF` writer - ADC10 Reference Select Bit: 0"]
pub type SrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sref, crate::Safe>;
impl<'a, REG> SrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VR+ = AVCC and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_0(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref0)
    }
    #[doc = "VR+ = VREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref1)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref2)
    }
    #[doc = "VR+ = VEREF+ and VR- = AVSS"]
    #[inline(always)]
    pub fn sref_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref3)
    }
    #[doc = "VR+ = AVCC and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref4)
    }
    #[doc = "VR+ = VREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_5(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref5)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_6(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref6)
    }
    #[doc = "VR+ = VEREF+ and VR- = VREF-/VEREF-"]
    #[inline(always)]
    pub fn sref_7(self) -> &'a mut crate::W<REG> {
        self.variant(Sref::Sref7)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&self) -> Adc10scR {
        Adc10scR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&self) -> Adc10ifgR {
        Adc10ifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&self) -> Adc10ieR {
        Adc10ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&self) -> Adc10onR {
        Adc10onR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&self) -> RefonR {
        RefonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&self) -> Ref2_5vR {
        Ref2_5vR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&self) -> RefburstR {
        RefburstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&self) -> RefoutR {
        RefoutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&self) -> Adc10srR {
        Adc10srR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&self) -> Adc10shtR {
        Adc10shtR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&self) -> SrefR {
        SrefR::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 Start Conversion"]
    #[inline(always)]
    pub fn adc10sc(&mut self) -> Adc10scW<Adc10ctl0Spec> {
        Adc10scW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC10 Enable Conversion"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<Adc10ctl0Spec> {
        EncW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC10 Interrupt Flag"]
    #[inline(always)]
    pub fn adc10ifg(&mut self) -> Adc10ifgW<Adc10ctl0Spec> {
        Adc10ifgW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC10 Interrupt Enalbe"]
    #[inline(always)]
    pub fn adc10ie(&mut self) -> Adc10ieW<Adc10ctl0Spec> {
        Adc10ieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC10 On/Enable"]
    #[inline(always)]
    pub fn adc10on(&mut self) -> Adc10onW<Adc10ctl0Spec> {
        Adc10onW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC10 Reference on"]
    #[inline(always)]
    pub fn refon(&mut self) -> RefonW<Adc10ctl0Spec> {
        RefonW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC10 Ref 0:1.5V / 1:2.5V"]
    #[inline(always)]
    pub fn ref2_5v(&mut self) -> Ref2_5vW<Adc10ctl0Spec> {
        Ref2_5vW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC10 Multiple SampleConversion"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<Adc10ctl0Spec> {
        MscW::new(self, 7)
    }
    #[doc = "Bit 8 - ADC10 Reference Burst Mode"]
    #[inline(always)]
    pub fn refburst(&mut self) -> RefburstW<Adc10ctl0Spec> {
        RefburstW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC10 Enalbe output of Ref."]
    #[inline(always)]
    pub fn refout(&mut self) -> RefoutW<Adc10ctl0Spec> {
        RefoutW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC10 Sampling Rate 0:200ksps / 1:50ksps"]
    #[inline(always)]
    pub fn adc10sr(&mut self) -> Adc10srW<Adc10ctl0Spec> {
        Adc10srW::new(self, 10)
    }
    #[doc = "Bits 11:12 - ADC10 Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adc10sht(&mut self) -> Adc10shtW<Adc10ctl0Spec> {
        Adc10shtW::new(self, 11)
    }
    #[doc = "Bits 13:15 - ADC10 Reference Select Bit: 0"]
    #[inline(always)]
    pub fn sref(&mut self) -> SrefW<Adc10ctl0Spec> {
        SrefW::new(self, 13)
    }
}
#[doc = "ADC10 Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10ctl0Spec;
impl crate::RegisterSpec for Adc10ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10ctl0::R`](R) reader structure"]
impl crate::Readable for Adc10ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10ctl0::W`](W) writer structure"]
impl crate::Writable for Adc10ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC10CTL0 to value 0"]
impl crate::Resettable for Adc10ctl0Spec {
    const RESET_VALUE: u16 = 0;
}
