#[doc = "Register `ADC10CTL1` reader"]
pub type R = crate::R<Adc10ctl1Spec>;
#[doc = "Register `ADC10CTL1` writer"]
pub type W = crate::W<Adc10ctl1Spec>;
#[doc = "Field `ADC10BUSY` reader - ADC10 BUSY"]
pub type Adc10busyR = crate::BitReader;
#[doc = "Field `ADC10BUSY` writer - ADC10 BUSY"]
pub type Adc10busyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC10 Conversion Sequence Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Conseq {
    #[doc = "0: Single channel single conversion"]
    Conseq0 = 0,
    #[doc = "1: Sequence of channels"]
    Conseq1 = 1,
    #[doc = "2: Repeat single channel"]
    Conseq2 = 2,
    #[doc = "3: Repeat sequence of channels"]
    Conseq3 = 3,
}
impl From<Conseq> for u8 {
    #[inline(always)]
    fn from(variant: Conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Conseq {
    type Ux = u8;
}
impl crate::IsEnum for Conseq {}
#[doc = "Field `CONSEQ` reader - ADC10 Conversion Sequence Select 0"]
pub type ConseqR = crate::FieldReader<Conseq>;
impl ConseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Conseq {
        match self.bits {
            0 => Conseq::Conseq0,
            1 => Conseq::Conseq1,
            2 => Conseq::Conseq2,
            3 => Conseq::Conseq3,
            _ => unreachable!(),
        }
    }
    #[doc = "Single channel single conversion"]
    #[inline(always)]
    pub fn is_conseq_0(&self) -> bool {
        *self == Conseq::Conseq0
    }
    #[doc = "Sequence of channels"]
    #[inline(always)]
    pub fn is_conseq_1(&self) -> bool {
        *self == Conseq::Conseq1
    }
    #[doc = "Repeat single channel"]
    #[inline(always)]
    pub fn is_conseq_2(&self) -> bool {
        *self == Conseq::Conseq2
    }
    #[doc = "Repeat sequence of channels"]
    #[inline(always)]
    pub fn is_conseq_3(&self) -> bool {
        *self == Conseq::Conseq3
    }
}
#[doc = "Field `CONSEQ` writer - ADC10 Conversion Sequence Select 0"]
pub type ConseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Conseq, crate::Safe>;
impl<'a, REG> ConseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single channel single conversion"]
    #[inline(always)]
    pub fn conseq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq0)
    }
    #[doc = "Sequence of channels"]
    #[inline(always)]
    pub fn conseq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq1)
    }
    #[doc = "Repeat single channel"]
    #[inline(always)]
    pub fn conseq_2(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq2)
    }
    #[doc = "Repeat sequence of channels"]
    #[inline(always)]
    pub fn conseq_3(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Conseq3)
    }
}
#[doc = "ADC10 Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc10ssel {
    #[doc = "0: ADC10OSC"]
    Adc10ssel0 = 0,
    #[doc = "1: ACLK"]
    Adc10ssel1 = 1,
    #[doc = "2: MCLK"]
    Adc10ssel2 = 2,
    #[doc = "3: SMCLK"]
    Adc10ssel3 = 3,
}
impl From<Adc10ssel> for u8 {
    #[inline(always)]
    fn from(variant: Adc10ssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc10ssel {
    type Ux = u8;
}
impl crate::IsEnum for Adc10ssel {}
#[doc = "Field `ADC10SSEL` reader - ADC10 Clock Source Select Bit: 0"]
pub type Adc10sselR = crate::FieldReader<Adc10ssel>;
impl Adc10sselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc10ssel {
        match self.bits {
            0 => Adc10ssel::Adc10ssel0,
            1 => Adc10ssel::Adc10ssel1,
            2 => Adc10ssel::Adc10ssel2,
            3 => Adc10ssel::Adc10ssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC10OSC"]
    #[inline(always)]
    pub fn is_adc10ssel_0(&self) -> bool {
        *self == Adc10ssel::Adc10ssel0
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_adc10ssel_1(&self) -> bool {
        *self == Adc10ssel::Adc10ssel1
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn is_adc10ssel_2(&self) -> bool {
        *self == Adc10ssel::Adc10ssel2
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_adc10ssel_3(&self) -> bool {
        *self == Adc10ssel::Adc10ssel3
    }
}
#[doc = "Field `ADC10SSEL` writer - ADC10 Clock Source Select Bit: 0"]
pub type Adc10sselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc10ssel, crate::Safe>;
impl<'a, REG> Adc10sselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC10OSC"]
    #[inline(always)]
    pub fn adc10ssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10ssel::Adc10ssel0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn adc10ssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10ssel::Adc10ssel1)
    }
    #[doc = "MCLK"]
    #[inline(always)]
    pub fn adc10ssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10ssel::Adc10ssel2)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn adc10ssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10ssel::Adc10ssel3)
    }
}
#[doc = "ADC10 Clock Divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc10div {
    #[doc = "0: ADC10 Clock Divider Select 0"]
    Adc10div0 = 0,
    #[doc = "1: ADC10 Clock Divider Select 1"]
    Adc10div1 = 1,
    #[doc = "2: ADC10 Clock Divider Select 2"]
    Adc10div2 = 2,
    #[doc = "3: ADC10 Clock Divider Select 3"]
    Adc10div3 = 3,
    #[doc = "4: ADC10 Clock Divider Select 4"]
    Adc10div4 = 4,
    #[doc = "5: ADC10 Clock Divider Select 5"]
    Adc10div5 = 5,
    #[doc = "6: ADC10 Clock Divider Select 6"]
    Adc10div6 = 6,
    #[doc = "7: ADC10 Clock Divider Select 7"]
    Adc10div7 = 7,
}
impl From<Adc10div> for u8 {
    #[inline(always)]
    fn from(variant: Adc10div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc10div {
    type Ux = u8;
}
impl crate::IsEnum for Adc10div {}
#[doc = "Field `ADC10DIV` reader - ADC10 Clock Divider Select Bit: 0"]
pub type Adc10divR = crate::FieldReader<Adc10div>;
impl Adc10divR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc10div {
        match self.bits {
            0 => Adc10div::Adc10div0,
            1 => Adc10div::Adc10div1,
            2 => Adc10div::Adc10div2,
            3 => Adc10div::Adc10div3,
            4 => Adc10div::Adc10div4,
            5 => Adc10div::Adc10div5,
            6 => Adc10div::Adc10div6,
            7 => Adc10div::Adc10div7,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC10 Clock Divider Select 0"]
    #[inline(always)]
    pub fn is_adc10div_0(&self) -> bool {
        *self == Adc10div::Adc10div0
    }
    #[doc = "ADC10 Clock Divider Select 1"]
    #[inline(always)]
    pub fn is_adc10div_1(&self) -> bool {
        *self == Adc10div::Adc10div1
    }
    #[doc = "ADC10 Clock Divider Select 2"]
    #[inline(always)]
    pub fn is_adc10div_2(&self) -> bool {
        *self == Adc10div::Adc10div2
    }
    #[doc = "ADC10 Clock Divider Select 3"]
    #[inline(always)]
    pub fn is_adc10div_3(&self) -> bool {
        *self == Adc10div::Adc10div3
    }
    #[doc = "ADC10 Clock Divider Select 4"]
    #[inline(always)]
    pub fn is_adc10div_4(&self) -> bool {
        *self == Adc10div::Adc10div4
    }
    #[doc = "ADC10 Clock Divider Select 5"]
    #[inline(always)]
    pub fn is_adc10div_5(&self) -> bool {
        *self == Adc10div::Adc10div5
    }
    #[doc = "ADC10 Clock Divider Select 6"]
    #[inline(always)]
    pub fn is_adc10div_6(&self) -> bool {
        *self == Adc10div::Adc10div6
    }
    #[doc = "ADC10 Clock Divider Select 7"]
    #[inline(always)]
    pub fn is_adc10div_7(&self) -> bool {
        *self == Adc10div::Adc10div7
    }
}
#[doc = "Field `ADC10DIV` writer - ADC10 Clock Divider Select Bit: 0"]
pub type Adc10divW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adc10div, crate::Safe>;
impl<'a, REG> Adc10divW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC10 Clock Divider Select 0"]
    #[inline(always)]
    pub fn adc10div_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div0)
    }
    #[doc = "ADC10 Clock Divider Select 1"]
    #[inline(always)]
    pub fn adc10div_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div1)
    }
    #[doc = "ADC10 Clock Divider Select 2"]
    #[inline(always)]
    pub fn adc10div_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div2)
    }
    #[doc = "ADC10 Clock Divider Select 3"]
    #[inline(always)]
    pub fn adc10div_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div3)
    }
    #[doc = "ADC10 Clock Divider Select 4"]
    #[inline(always)]
    pub fn adc10div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div4)
    }
    #[doc = "ADC10 Clock Divider Select 5"]
    #[inline(always)]
    pub fn adc10div_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div5)
    }
    #[doc = "ADC10 Clock Divider Select 6"]
    #[inline(always)]
    pub fn adc10div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div6)
    }
    #[doc = "ADC10 Clock Divider Select 7"]
    #[inline(always)]
    pub fn adc10div_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adc10div::Adc10div7)
    }
}
#[doc = "Field `ISSH` reader - ADC10 Invert Sample Hold Signal"]
pub type IsshR = crate::BitReader;
#[doc = "Field `ISSH` writer - ADC10 Invert Sample Hold Signal"]
pub type IsshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC10DF` reader - ADC10 Data Format 0:binary 1:2's complement"]
pub type Adc10dfR = crate::BitReader;
#[doc = "Field `ADC10DF` writer - ADC10 Data Format 0:binary 1:2's complement"]
pub type Adc10dfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC10 Sample/Hold Source Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Shs {
    #[doc = "0: ADC10SC"]
    Shs0 = 0,
    #[doc = "1: TA3 OUT1"]
    Shs1 = 1,
    #[doc = "2: TA3 OUT0"]
    Shs2 = 2,
    #[doc = "3: TA3 OUT2"]
    Shs3 = 3,
}
impl From<Shs> for u8 {
    #[inline(always)]
    fn from(variant: Shs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Shs {
    type Ux = u8;
}
impl crate::IsEnum for Shs {}
#[doc = "Field `SHS` reader - ADC10 Sample/Hold Source Bit: 0"]
pub type ShsR = crate::FieldReader<Shs>;
impl ShsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shs {
        match self.bits {
            0 => Shs::Shs0,
            1 => Shs::Shs1,
            2 => Shs::Shs2,
            3 => Shs::Shs3,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC10SC"]
    #[inline(always)]
    pub fn is_shs_0(&self) -> bool {
        *self == Shs::Shs0
    }
    #[doc = "TA3 OUT1"]
    #[inline(always)]
    pub fn is_shs_1(&self) -> bool {
        *self == Shs::Shs1
    }
    #[doc = "TA3 OUT0"]
    #[inline(always)]
    pub fn is_shs_2(&self) -> bool {
        *self == Shs::Shs2
    }
    #[doc = "TA3 OUT2"]
    #[inline(always)]
    pub fn is_shs_3(&self) -> bool {
        *self == Shs::Shs3
    }
}
#[doc = "Field `SHS` writer - ADC10 Sample/Hold Source Bit: 0"]
pub type ShsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Shs, crate::Safe>;
impl<'a, REG> ShsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC10SC"]
    #[inline(always)]
    pub fn shs_0(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs0)
    }
    #[doc = "TA3 OUT1"]
    #[inline(always)]
    pub fn shs_1(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs1)
    }
    #[doc = "TA3 OUT0"]
    #[inline(always)]
    pub fn shs_2(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs2)
    }
    #[doc = "TA3 OUT2"]
    #[inline(always)]
    pub fn shs_3(self) -> &'a mut crate::W<REG> {
        self.variant(Shs::Shs3)
    }
}
#[doc = "ADC10 Input Channel Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inch {
    #[doc = "0: Selects Channel 0"]
    Inch0 = 0,
    #[doc = "1: Selects Channel 1"]
    Inch1 = 1,
    #[doc = "2: Selects Channel 2"]
    Inch2 = 2,
    #[doc = "3: Selects Channel 3"]
    Inch3 = 3,
    #[doc = "4: Selects Channel 4"]
    Inch4 = 4,
    #[doc = "5: Selects Channel 5"]
    Inch5 = 5,
    #[doc = "6: Selects Channel 6"]
    Inch6 = 6,
    #[doc = "7: Selects Channel 7"]
    Inch7 = 7,
    #[doc = "8: Selects Channel 8"]
    Inch8 = 8,
    #[doc = "9: Selects Channel 9"]
    Inch9 = 9,
    #[doc = "10: Selects Channel 10"]
    Inch10 = 10,
    #[doc = "11: Selects Channel 11"]
    Inch11 = 11,
    #[doc = "12: Selects Channel 12"]
    Inch12 = 12,
    #[doc = "13: Selects Channel 13"]
    Inch13 = 13,
    #[doc = "14: Selects Channel 14"]
    Inch14 = 14,
    #[doc = "15: Selects Channel 15"]
    Inch15 = 15,
}
impl From<Inch> for u8 {
    #[inline(always)]
    fn from(variant: Inch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inch {
    type Ux = u8;
}
impl crate::IsEnum for Inch {}
#[doc = "Field `INCH` reader - ADC10 Input Channel Select Bit: 0"]
pub type InchR = crate::FieldReader<Inch>;
impl InchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inch {
        match self.bits {
            0 => Inch::Inch0,
            1 => Inch::Inch1,
            2 => Inch::Inch2,
            3 => Inch::Inch3,
            4 => Inch::Inch4,
            5 => Inch::Inch5,
            6 => Inch::Inch6,
            7 => Inch::Inch7,
            8 => Inch::Inch8,
            9 => Inch::Inch9,
            10 => Inch::Inch10,
            11 => Inch::Inch11,
            12 => Inch::Inch12,
            13 => Inch::Inch13,
            14 => Inch::Inch14,
            15 => Inch::Inch15,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects Channel 0"]
    #[inline(always)]
    pub fn is_inch_0(&self) -> bool {
        *self == Inch::Inch0
    }
    #[doc = "Selects Channel 1"]
    #[inline(always)]
    pub fn is_inch_1(&self) -> bool {
        *self == Inch::Inch1
    }
    #[doc = "Selects Channel 2"]
    #[inline(always)]
    pub fn is_inch_2(&self) -> bool {
        *self == Inch::Inch2
    }
    #[doc = "Selects Channel 3"]
    #[inline(always)]
    pub fn is_inch_3(&self) -> bool {
        *self == Inch::Inch3
    }
    #[doc = "Selects Channel 4"]
    #[inline(always)]
    pub fn is_inch_4(&self) -> bool {
        *self == Inch::Inch4
    }
    #[doc = "Selects Channel 5"]
    #[inline(always)]
    pub fn is_inch_5(&self) -> bool {
        *self == Inch::Inch5
    }
    #[doc = "Selects Channel 6"]
    #[inline(always)]
    pub fn is_inch_6(&self) -> bool {
        *self == Inch::Inch6
    }
    #[doc = "Selects Channel 7"]
    #[inline(always)]
    pub fn is_inch_7(&self) -> bool {
        *self == Inch::Inch7
    }
    #[doc = "Selects Channel 8"]
    #[inline(always)]
    pub fn is_inch_8(&self) -> bool {
        *self == Inch::Inch8
    }
    #[doc = "Selects Channel 9"]
    #[inline(always)]
    pub fn is_inch_9(&self) -> bool {
        *self == Inch::Inch9
    }
    #[doc = "Selects Channel 10"]
    #[inline(always)]
    pub fn is_inch_10(&self) -> bool {
        *self == Inch::Inch10
    }
    #[doc = "Selects Channel 11"]
    #[inline(always)]
    pub fn is_inch_11(&self) -> bool {
        *self == Inch::Inch11
    }
    #[doc = "Selects Channel 12"]
    #[inline(always)]
    pub fn is_inch_12(&self) -> bool {
        *self == Inch::Inch12
    }
    #[doc = "Selects Channel 13"]
    #[inline(always)]
    pub fn is_inch_13(&self) -> bool {
        *self == Inch::Inch13
    }
    #[doc = "Selects Channel 14"]
    #[inline(always)]
    pub fn is_inch_14(&self) -> bool {
        *self == Inch::Inch14
    }
    #[doc = "Selects Channel 15"]
    #[inline(always)]
    pub fn is_inch_15(&self) -> bool {
        *self == Inch::Inch15
    }
}
#[doc = "Field `INCH` writer - ADC10 Input Channel Select Bit: 0"]
pub type InchW<'a, REG> = crate::FieldWriter<'a, REG, 4, Inch, crate::Safe>;
impl<'a, REG> InchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects Channel 0"]
    #[inline(always)]
    pub fn inch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch0)
    }
    #[doc = "Selects Channel 1"]
    #[inline(always)]
    pub fn inch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch1)
    }
    #[doc = "Selects Channel 2"]
    #[inline(always)]
    pub fn inch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch2)
    }
    #[doc = "Selects Channel 3"]
    #[inline(always)]
    pub fn inch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch3)
    }
    #[doc = "Selects Channel 4"]
    #[inline(always)]
    pub fn inch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch4)
    }
    #[doc = "Selects Channel 5"]
    #[inline(always)]
    pub fn inch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch5)
    }
    #[doc = "Selects Channel 6"]
    #[inline(always)]
    pub fn inch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch6)
    }
    #[doc = "Selects Channel 7"]
    #[inline(always)]
    pub fn inch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch7)
    }
    #[doc = "Selects Channel 8"]
    #[inline(always)]
    pub fn inch_8(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch8)
    }
    #[doc = "Selects Channel 9"]
    #[inline(always)]
    pub fn inch_9(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch9)
    }
    #[doc = "Selects Channel 10"]
    #[inline(always)]
    pub fn inch_10(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch10)
    }
    #[doc = "Selects Channel 11"]
    #[inline(always)]
    pub fn inch_11(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch11)
    }
    #[doc = "Selects Channel 12"]
    #[inline(always)]
    pub fn inch_12(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch12)
    }
    #[doc = "Selects Channel 13"]
    #[inline(always)]
    pub fn inch_13(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch13)
    }
    #[doc = "Selects Channel 14"]
    #[inline(always)]
    pub fn inch_14(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch14)
    }
    #[doc = "Selects Channel 15"]
    #[inline(always)]
    pub fn inch_15(self) -> &'a mut crate::W<REG> {
        self.variant(Inch::Inch15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC10 BUSY"]
    #[inline(always)]
    pub fn adc10busy(&self) -> Adc10busyR {
        Adc10busyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn conseq(&self) -> ConseqR {
        ConseqR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc10ssel(&self) -> Adc10sselR {
        Adc10sselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc10div(&self) -> Adc10divR {
        Adc10divR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn issh(&self) -> IsshR {
        IsshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC10 Data Format 0:binary 1:2's complement"]
    #[inline(always)]
    pub fn adc10df(&self) -> Adc10dfR {
        Adc10dfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn shs(&self) -> ShsR {
        ShsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - ADC10 Input Channel Select Bit: 0"]
    #[inline(always)]
    pub fn inch(&self) -> InchR {
        InchR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC10 BUSY"]
    #[inline(always)]
    pub fn adc10busy(&mut self) -> Adc10busyW<Adc10ctl1Spec> {
        Adc10busyW::new(self, 0)
    }
    #[doc = "Bits 1:2 - ADC10 Conversion Sequence Select 0"]
    #[inline(always)]
    pub fn conseq(&mut self) -> ConseqW<Adc10ctl1Spec> {
        ConseqW::new(self, 1)
    }
    #[doc = "Bits 3:4 - ADC10 Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn adc10ssel(&mut self) -> Adc10sselW<Adc10ctl1Spec> {
        Adc10sselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - ADC10 Clock Divider Select Bit: 0"]
    #[inline(always)]
    pub fn adc10div(&mut self) -> Adc10divW<Adc10ctl1Spec> {
        Adc10divW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC10 Invert Sample Hold Signal"]
    #[inline(always)]
    pub fn issh(&mut self) -> IsshW<Adc10ctl1Spec> {
        IsshW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC10 Data Format 0:binary 1:2's complement"]
    #[inline(always)]
    pub fn adc10df(&mut self) -> Adc10dfW<Adc10ctl1Spec> {
        Adc10dfW::new(self, 9)
    }
    #[doc = "Bits 10:11 - ADC10 Sample/Hold Source Bit: 0"]
    #[inline(always)]
    pub fn shs(&mut self) -> ShsW<Adc10ctl1Spec> {
        ShsW::new(self, 10)
    }
    #[doc = "Bits 12:15 - ADC10 Input Channel Select Bit: 0"]
    #[inline(always)]
    pub fn inch(&mut self) -> InchW<Adc10ctl1Spec> {
        InchW::new(self, 12)
    }
}
#[doc = "ADC10 Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc10ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc10ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc10ctl1Spec;
impl crate::RegisterSpec for Adc10ctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adc10ctl1::R`](R) reader structure"]
impl crate::Readable for Adc10ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc10ctl1::W`](W) writer structure"]
impl crate::Writable for Adc10ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADC10CTL1 to value 0"]
impl crate::Resettable for Adc10ctl1Spec {
    const RESET_VALUE: u16 = 0;
}
