#[doc = "Register `CACTL2` reader"]
pub type R = crate::R<Cactl2Spec>;
#[doc = "Register `CACTL2` writer"]
pub type W = crate::W<Cactl2Spec>;
#[doc = "Field `CAOUT` reader - Comp. A Output"]
pub type CaoutR = crate::BitReader;
#[doc = "Field `CAOUT` writer - Comp. A Output"]
pub type CaoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Comp. A Enable Output Filter"]
pub type CafR = crate::BitReader;
#[doc = "Field `CAF` writer - Comp. A Enable Output Filter"]
pub type CafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comp. A +Terminal Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2ca {
    #[doc = "0: No + or - connection"]
    NoneNone = 0,
    #[doc = "1: Connect + to CA0, no connection to -"]
    Ca0None = 1,
    #[doc = "2: No connection to +, connect - to CA1"]
    NoneCa1 = 2,
    #[doc = "3: Connect + to CA0, connect - to CA1"]
    Ca0Ca1 = 3,
    #[doc = "4: No connection to +, connect - to CA2"]
    NoneCa2 = 4,
    #[doc = "5: Connect + to CA0, connect - to CA2"]
    Ca0Ca2 = 5,
    #[doc = "6: No connection to +, connect - to CA3"]
    NoneCa3 = 6,
    #[doc = "7: Connect + to CA0, connect - to CA3"]
    Ca0Ca3 = 7,
    #[doc = "8: No connection to +, connect - to CA4"]
    NoneCa4 = 8,
    #[doc = "9: Connect + to CA0, connect - to CA4"]
    Ca0Ca4 = 9,
    #[doc = "10: No connection to +, connect - to CA5"]
    NoneCa5 = 10,
    #[doc = "11: Connect + to CA0, connect - to CA5"]
    Ca0Ca5 = 11,
    #[doc = "12: No connection to +, connect - to CA6"]
    NoneCa6 = 12,
    #[doc = "13: Connect + to CA0, connect - to CA6"]
    Ca0Ca6 = 13,
    #[doc = "14: No connection to +, connect - to CA7"]
    NoneCa7 = 14,
    #[doc = "15: Connect + to CA0, connect - to CA7"]
    Ca0Ca7 = 15,
    #[doc = "16: Connect + to CA1, no connection to -"]
    Ca1None = 16,
    #[doc = "17: Connect + to CA2, no connection to -"]
    Ca2None = 17,
    #[doc = "18: Connect + to CA1, connect - to CA1"]
    Ca1Ca1 = 18,
    #[doc = "19: Connect + to CA2, connect - to CA1"]
    Ca2Ca1 = 19,
    #[doc = "20: Connect + to CA1, connect - to CA2"]
    Ca1Ca2 = 20,
    #[doc = "21: Connect + to CA2, connect - to CA2"]
    Ca2Ca2 = 21,
    #[doc = "22: Connect + to CA1, connect - to CA3"]
    Ca1Ca3 = 22,
    #[doc = "23: Connect + to CA2, connect - to CA3"]
    Ca2Ca3 = 23,
    #[doc = "24: Connect + to CA1, connect - to CA4"]
    Ca1Ca4 = 24,
    #[doc = "25: Connect + to CA2, connect - to CA4"]
    Ca2Ca4 = 25,
    #[doc = "26: Connect + to CA1, connect - to CA5"]
    Ca1Ca5 = 26,
    #[doc = "27: Connect + to CA2, connect - to CA5"]
    Ca2Ca5 = 27,
    #[doc = "28: Connect + to CA1, connect - to CA6"]
    Ca1Ca6 = 28,
    #[doc = "29: Connect + to CA2, connect - to CA6"]
    Ca2Ca6 = 29,
    #[doc = "30: Connect + to CA1, connect - to CA7"]
    Ca1Ca7 = 30,
    #[doc = "31: Connect + to CA2, connect - to CA7"]
    Ca2Ca7 = 31,
}
impl From<P2ca> for u8 {
    #[inline(always)]
    fn from(variant: P2ca) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2ca {
    type Ux = u8;
}
impl crate::IsEnum for P2ca {}
#[doc = "Field `P2CA` reader - Comp. A +Terminal Multiplexer"]
pub type P2caR = crate::FieldReader<P2ca>;
impl P2caR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2ca {
        match self.bits {
            0 => P2ca::NoneNone,
            1 => P2ca::Ca0None,
            2 => P2ca::NoneCa1,
            3 => P2ca::Ca0Ca1,
            4 => P2ca::NoneCa2,
            5 => P2ca::Ca0Ca2,
            6 => P2ca::NoneCa3,
            7 => P2ca::Ca0Ca3,
            8 => P2ca::NoneCa4,
            9 => P2ca::Ca0Ca4,
            10 => P2ca::NoneCa5,
            11 => P2ca::Ca0Ca5,
            12 => P2ca::NoneCa6,
            13 => P2ca::Ca0Ca6,
            14 => P2ca::NoneCa7,
            15 => P2ca::Ca0Ca7,
            16 => P2ca::Ca1None,
            17 => P2ca::Ca2None,
            18 => P2ca::Ca1Ca1,
            19 => P2ca::Ca2Ca1,
            20 => P2ca::Ca1Ca2,
            21 => P2ca::Ca2Ca2,
            22 => P2ca::Ca1Ca3,
            23 => P2ca::Ca2Ca3,
            24 => P2ca::Ca1Ca4,
            25 => P2ca::Ca2Ca4,
            26 => P2ca::Ca1Ca5,
            27 => P2ca::Ca2Ca5,
            28 => P2ca::Ca1Ca6,
            29 => P2ca::Ca2Ca6,
            30 => P2ca::Ca1Ca7,
            31 => P2ca::Ca2Ca7,
            _ => unreachable!(),
        }
    }
    #[doc = "No + or - connection"]
    #[inline(always)]
    pub fn is_none_none(&self) -> bool {
        *self == P2ca::NoneNone
    }
    #[doc = "Connect + to CA0, no connection to -"]
    #[inline(always)]
    pub fn is_ca0_none(&self) -> bool {
        *self == P2ca::Ca0None
    }
    #[doc = "No connection to +, connect - to CA1"]
    #[inline(always)]
    pub fn is_none_ca1(&self) -> bool {
        *self == P2ca::NoneCa1
    }
    #[doc = "Connect + to CA0, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca0_ca1(&self) -> bool {
        *self == P2ca::Ca0Ca1
    }
    #[doc = "No connection to +, connect - to CA2"]
    #[inline(always)]
    pub fn is_none_ca2(&self) -> bool {
        *self == P2ca::NoneCa2
    }
    #[doc = "Connect + to CA0, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca0_ca2(&self) -> bool {
        *self == P2ca::Ca0Ca2
    }
    #[doc = "No connection to +, connect - to CA3"]
    #[inline(always)]
    pub fn is_none_ca3(&self) -> bool {
        *self == P2ca::NoneCa3
    }
    #[doc = "Connect + to CA0, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca0_ca3(&self) -> bool {
        *self == P2ca::Ca0Ca3
    }
    #[doc = "No connection to +, connect - to CA4"]
    #[inline(always)]
    pub fn is_none_ca4(&self) -> bool {
        *self == P2ca::NoneCa4
    }
    #[doc = "Connect + to CA0, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca0_ca4(&self) -> bool {
        *self == P2ca::Ca0Ca4
    }
    #[doc = "No connection to +, connect - to CA5"]
    #[inline(always)]
    pub fn is_none_ca5(&self) -> bool {
        *self == P2ca::NoneCa5
    }
    #[doc = "Connect + to CA0, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca0_ca5(&self) -> bool {
        *self == P2ca::Ca0Ca5
    }
    #[doc = "No connection to +, connect - to CA6"]
    #[inline(always)]
    pub fn is_none_ca6(&self) -> bool {
        *self == P2ca::NoneCa6
    }
    #[doc = "Connect + to CA0, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca0_ca6(&self) -> bool {
        *self == P2ca::Ca0Ca6
    }
    #[doc = "No connection to +, connect - to CA7"]
    #[inline(always)]
    pub fn is_none_ca7(&self) -> bool {
        *self == P2ca::NoneCa7
    }
    #[doc = "Connect + to CA0, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca0_ca7(&self) -> bool {
        *self == P2ca::Ca0Ca7
    }
    #[doc = "Connect + to CA1, no connection to -"]
    #[inline(always)]
    pub fn is_ca1_none(&self) -> bool {
        *self == P2ca::Ca1None
    }
    #[doc = "Connect + to CA2, no connection to -"]
    #[inline(always)]
    pub fn is_ca2_none(&self) -> bool {
        *self == P2ca::Ca2None
    }
    #[doc = "Connect + to CA1, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca1_ca1(&self) -> bool {
        *self == P2ca::Ca1Ca1
    }
    #[doc = "Connect + to CA2, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca2_ca1(&self) -> bool {
        *self == P2ca::Ca2Ca1
    }
    #[doc = "Connect + to CA1, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca1_ca2(&self) -> bool {
        *self == P2ca::Ca1Ca2
    }
    #[doc = "Connect + to CA2, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca2_ca2(&self) -> bool {
        *self == P2ca::Ca2Ca2
    }
    #[doc = "Connect + to CA1, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca1_ca3(&self) -> bool {
        *self == P2ca::Ca1Ca3
    }
    #[doc = "Connect + to CA2, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca2_ca3(&self) -> bool {
        *self == P2ca::Ca2Ca3
    }
    #[doc = "Connect + to CA1, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca1_ca4(&self) -> bool {
        *self == P2ca::Ca1Ca4
    }
    #[doc = "Connect + to CA2, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca2_ca4(&self) -> bool {
        *self == P2ca::Ca2Ca4
    }
    #[doc = "Connect + to CA1, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca1_ca5(&self) -> bool {
        *self == P2ca::Ca1Ca5
    }
    #[doc = "Connect + to CA2, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca2_ca5(&self) -> bool {
        *self == P2ca::Ca2Ca5
    }
    #[doc = "Connect + to CA1, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca1_ca6(&self) -> bool {
        *self == P2ca::Ca1Ca6
    }
    #[doc = "Connect + to CA2, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca2_ca6(&self) -> bool {
        *self == P2ca::Ca2Ca6
    }
    #[doc = "Connect + to CA1, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca1_ca7(&self) -> bool {
        *self == P2ca::Ca1Ca7
    }
    #[doc = "Connect + to CA2, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca2_ca7(&self) -> bool {
        *self == P2ca::Ca2Ca7
    }
}
#[doc = "Field `P2CA` writer - Comp. A +Terminal Multiplexer"]
pub type P2caW<'a, REG> = crate::FieldWriter<'a, REG, 5, P2ca, crate::Safe>;
impl<'a, REG> P2caW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No + or - connection"]
    #[inline(always)]
    pub fn none_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneNone)
    }
    #[doc = "Connect + to CA0, no connection to -"]
    #[inline(always)]
    pub fn ca0_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0None)
    }
    #[doc = "No connection to +, connect - to CA1"]
    #[inline(always)]
    pub fn none_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa1)
    }
    #[doc = "Connect + to CA0, connect - to CA1"]
    #[inline(always)]
    pub fn ca0_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca1)
    }
    #[doc = "No connection to +, connect - to CA2"]
    #[inline(always)]
    pub fn none_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa2)
    }
    #[doc = "Connect + to CA0, connect - to CA2"]
    #[inline(always)]
    pub fn ca0_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca2)
    }
    #[doc = "No connection to +, connect - to CA3"]
    #[inline(always)]
    pub fn none_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa3)
    }
    #[doc = "Connect + to CA0, connect - to CA3"]
    #[inline(always)]
    pub fn ca0_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca3)
    }
    #[doc = "No connection to +, connect - to CA4"]
    #[inline(always)]
    pub fn none_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa4)
    }
    #[doc = "Connect + to CA0, connect - to CA4"]
    #[inline(always)]
    pub fn ca0_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca4)
    }
    #[doc = "No connection to +, connect - to CA5"]
    #[inline(always)]
    pub fn none_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa5)
    }
    #[doc = "Connect + to CA0, connect - to CA5"]
    #[inline(always)]
    pub fn ca0_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca5)
    }
    #[doc = "No connection to +, connect - to CA6"]
    #[inline(always)]
    pub fn none_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa6)
    }
    #[doc = "Connect + to CA0, connect - to CA6"]
    #[inline(always)]
    pub fn ca0_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca6)
    }
    #[doc = "No connection to +, connect - to CA7"]
    #[inline(always)]
    pub fn none_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::NoneCa7)
    }
    #[doc = "Connect + to CA0, connect - to CA7"]
    #[inline(always)]
    pub fn ca0_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca0Ca7)
    }
    #[doc = "Connect + to CA1, no connection to -"]
    #[inline(always)]
    pub fn ca1_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1None)
    }
    #[doc = "Connect + to CA2, no connection to -"]
    #[inline(always)]
    pub fn ca2_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2None)
    }
    #[doc = "Connect + to CA1, connect - to CA1"]
    #[inline(always)]
    pub fn ca1_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca1)
    }
    #[doc = "Connect + to CA2, connect - to CA1"]
    #[inline(always)]
    pub fn ca2_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca1)
    }
    #[doc = "Connect + to CA1, connect - to CA2"]
    #[inline(always)]
    pub fn ca1_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca2)
    }
    #[doc = "Connect + to CA2, connect - to CA2"]
    #[inline(always)]
    pub fn ca2_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca2)
    }
    #[doc = "Connect + to CA1, connect - to CA3"]
    #[inline(always)]
    pub fn ca1_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca3)
    }
    #[doc = "Connect + to CA2, connect - to CA3"]
    #[inline(always)]
    pub fn ca2_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca3)
    }
    #[doc = "Connect + to CA1, connect - to CA4"]
    #[inline(always)]
    pub fn ca1_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca4)
    }
    #[doc = "Connect + to CA2, connect - to CA4"]
    #[inline(always)]
    pub fn ca2_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca4)
    }
    #[doc = "Connect + to CA1, connect - to CA5"]
    #[inline(always)]
    pub fn ca1_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca5)
    }
    #[doc = "Connect + to CA2, connect - to CA5"]
    #[inline(always)]
    pub fn ca2_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca5)
    }
    #[doc = "Connect + to CA1, connect - to CA6"]
    #[inline(always)]
    pub fn ca1_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca6)
    }
    #[doc = "Connect + to CA2, connect - to CA6"]
    #[inline(always)]
    pub fn ca2_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca6)
    }
    #[doc = "Connect + to CA1, connect - to CA7"]
    #[inline(always)]
    pub fn ca1_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca1Ca7)
    }
    #[doc = "Connect + to CA2, connect - to CA7"]
    #[inline(always)]
    pub fn ca2_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2ca::Ca2Ca7)
    }
}
#[doc = "Field `CASHORT` reader - Comp. A Short + and - Terminals"]
pub type CashortR = crate::BitReader;
#[doc = "Field `CASHORT` writer - Comp. A Short + and - Terminals"]
pub type CashortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CaoutR {
        CaoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CafR {
        CafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&self) -> P2caR {
        P2caR::new((self.bits >> 2) & 0x1f)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CashortR {
        CashortR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&mut self) -> CaoutW<Cactl2Spec> {
        CaoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&mut self) -> CafW<Cactl2Spec> {
        CafW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&mut self) -> P2caW<Cactl2Spec> {
        P2caW::new(self, 2)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&mut self) -> CashortW<Cactl2Spec> {
        CashortW::new(self, 7)
    }
}
#[doc = "Comparator A Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cactl2Spec;
impl crate::RegisterSpec for Cactl2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cactl2::R`](R) reader structure"]
impl crate::Readable for Cactl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cactl2::W`](W) writer structure"]
impl crate::Writable for Cactl2Spec {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CACTL2 to value 0"]
impl crate::Resettable for Cactl2Spec {
    const RESET_VALUE: u8 = 0;
}
