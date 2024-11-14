#[doc = "Register `CACTL2` reader"]
pub type R = crate::R<CACTL2_SPEC>;
#[doc = "Register `CACTL2` writer"]
pub type W = crate::W<CACTL2_SPEC>;
#[doc = "Field `CAOUT` reader - Comp. A Output"]
pub type CAOUT_R = crate::BitReader;
#[doc = "Field `CAOUT` writer - Comp. A Output"]
pub type CAOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAF` reader - Comp. A Enable Output Filter"]
pub type CAF_R = crate::BitReader;
#[doc = "Field `CAF` writer - Comp. A Enable Output Filter"]
pub type CAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comp. A +Terminal Multiplexer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P2CA_A {
    #[doc = "0: No + or - connection"]
    NONE_NONE = 0,
    #[doc = "1: Connect + to CA0, no connection to -"]
    CA0_NONE = 1,
    #[doc = "2: No connection to +, connect - to CA1"]
    NONE_CA1 = 2,
    #[doc = "3: Connect + to CA0, connect - to CA1"]
    CA0_CA1 = 3,
    #[doc = "4: No connection to +, connect - to CA2"]
    NONE_CA2 = 4,
    #[doc = "5: Connect + to CA0, connect - to CA2"]
    CA0_CA2 = 5,
    #[doc = "6: No connection to +, connect - to CA3"]
    NONE_CA3 = 6,
    #[doc = "7: Connect + to CA0, connect - to CA3"]
    CA0_CA3 = 7,
    #[doc = "8: No connection to +, connect - to CA4"]
    NONE_CA4 = 8,
    #[doc = "9: Connect + to CA0, connect - to CA4"]
    CA0_CA4 = 9,
    #[doc = "10: No connection to +, connect - to CA5"]
    NONE_CA5 = 10,
    #[doc = "11: Connect + to CA0, connect - to CA5"]
    CA0_CA5 = 11,
    #[doc = "12: No connection to +, connect - to CA6"]
    NONE_CA6 = 12,
    #[doc = "13: Connect + to CA0, connect - to CA6"]
    CA0_CA6 = 13,
    #[doc = "14: No connection to +, connect - to CA7"]
    NONE_CA7 = 14,
    #[doc = "15: Connect + to CA0, connect - to CA7"]
    CA0_CA7 = 15,
    #[doc = "16: Connect + to CA1, no connection to -"]
    CA1_NONE = 16,
    #[doc = "17: Connect + to CA2, no connection to -"]
    CA2_NONE = 17,
    #[doc = "18: Connect + to CA1, connect - to CA1"]
    CA1_CA1 = 18,
    #[doc = "19: Connect + to CA2, connect - to CA1"]
    CA2_CA1 = 19,
    #[doc = "20: Connect + to CA1, connect - to CA2"]
    CA1_CA2 = 20,
    #[doc = "21: Connect + to CA2, connect - to CA2"]
    CA2_CA2 = 21,
    #[doc = "22: Connect + to CA1, connect - to CA3"]
    CA1_CA3 = 22,
    #[doc = "23: Connect + to CA2, connect - to CA3"]
    CA2_CA3 = 23,
    #[doc = "24: Connect + to CA1, connect - to CA4"]
    CA1_CA4 = 24,
    #[doc = "25: Connect + to CA2, connect - to CA4"]
    CA2_CA4 = 25,
    #[doc = "26: Connect + to CA1, connect - to CA5"]
    CA1_CA5 = 26,
    #[doc = "27: Connect + to CA2, connect - to CA5"]
    CA2_CA5 = 27,
    #[doc = "28: Connect + to CA1, connect - to CA6"]
    CA1_CA6 = 28,
    #[doc = "29: Connect + to CA2, connect - to CA6"]
    CA2_CA6 = 29,
    #[doc = "30: Connect + to CA1, connect - to CA7"]
    CA1_CA7 = 30,
    #[doc = "31: Connect + to CA2, connect - to CA7"]
    CA2_CA7 = 31,
}
impl From<P2CA_A> for u8 {
    #[inline(always)]
    fn from(variant: P2CA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for P2CA_A {
    type Ux = u8;
}
impl crate::IsEnum for P2CA_A {}
#[doc = "Field `P2CA` reader - Comp. A +Terminal Multiplexer"]
pub type P2CA_R = crate::FieldReader<P2CA_A>;
impl P2CA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2CA_A {
        match self.bits {
            0 => P2CA_A::NONE_NONE,
            1 => P2CA_A::CA0_NONE,
            2 => P2CA_A::NONE_CA1,
            3 => P2CA_A::CA0_CA1,
            4 => P2CA_A::NONE_CA2,
            5 => P2CA_A::CA0_CA2,
            6 => P2CA_A::NONE_CA3,
            7 => P2CA_A::CA0_CA3,
            8 => P2CA_A::NONE_CA4,
            9 => P2CA_A::CA0_CA4,
            10 => P2CA_A::NONE_CA5,
            11 => P2CA_A::CA0_CA5,
            12 => P2CA_A::NONE_CA6,
            13 => P2CA_A::CA0_CA6,
            14 => P2CA_A::NONE_CA7,
            15 => P2CA_A::CA0_CA7,
            16 => P2CA_A::CA1_NONE,
            17 => P2CA_A::CA2_NONE,
            18 => P2CA_A::CA1_CA1,
            19 => P2CA_A::CA2_CA1,
            20 => P2CA_A::CA1_CA2,
            21 => P2CA_A::CA2_CA2,
            22 => P2CA_A::CA1_CA3,
            23 => P2CA_A::CA2_CA3,
            24 => P2CA_A::CA1_CA4,
            25 => P2CA_A::CA2_CA4,
            26 => P2CA_A::CA1_CA5,
            27 => P2CA_A::CA2_CA5,
            28 => P2CA_A::CA1_CA6,
            29 => P2CA_A::CA2_CA6,
            30 => P2CA_A::CA1_CA7,
            31 => P2CA_A::CA2_CA7,
            _ => unreachable!(),
        }
    }
    #[doc = "No + or - connection"]
    #[inline(always)]
    pub fn is_none_none(&self) -> bool {
        *self == P2CA_A::NONE_NONE
    }
    #[doc = "Connect + to CA0, no connection to -"]
    #[inline(always)]
    pub fn is_ca0_none(&self) -> bool {
        *self == P2CA_A::CA0_NONE
    }
    #[doc = "No connection to +, connect - to CA1"]
    #[inline(always)]
    pub fn is_none_ca1(&self) -> bool {
        *self == P2CA_A::NONE_CA1
    }
    #[doc = "Connect + to CA0, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca0_ca1(&self) -> bool {
        *self == P2CA_A::CA0_CA1
    }
    #[doc = "No connection to +, connect - to CA2"]
    #[inline(always)]
    pub fn is_none_ca2(&self) -> bool {
        *self == P2CA_A::NONE_CA2
    }
    #[doc = "Connect + to CA0, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca0_ca2(&self) -> bool {
        *self == P2CA_A::CA0_CA2
    }
    #[doc = "No connection to +, connect - to CA3"]
    #[inline(always)]
    pub fn is_none_ca3(&self) -> bool {
        *self == P2CA_A::NONE_CA3
    }
    #[doc = "Connect + to CA0, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca0_ca3(&self) -> bool {
        *self == P2CA_A::CA0_CA3
    }
    #[doc = "No connection to +, connect - to CA4"]
    #[inline(always)]
    pub fn is_none_ca4(&self) -> bool {
        *self == P2CA_A::NONE_CA4
    }
    #[doc = "Connect + to CA0, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca0_ca4(&self) -> bool {
        *self == P2CA_A::CA0_CA4
    }
    #[doc = "No connection to +, connect - to CA5"]
    #[inline(always)]
    pub fn is_none_ca5(&self) -> bool {
        *self == P2CA_A::NONE_CA5
    }
    #[doc = "Connect + to CA0, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca0_ca5(&self) -> bool {
        *self == P2CA_A::CA0_CA5
    }
    #[doc = "No connection to +, connect - to CA6"]
    #[inline(always)]
    pub fn is_none_ca6(&self) -> bool {
        *self == P2CA_A::NONE_CA6
    }
    #[doc = "Connect + to CA0, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca0_ca6(&self) -> bool {
        *self == P2CA_A::CA0_CA6
    }
    #[doc = "No connection to +, connect - to CA7"]
    #[inline(always)]
    pub fn is_none_ca7(&self) -> bool {
        *self == P2CA_A::NONE_CA7
    }
    #[doc = "Connect + to CA0, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca0_ca7(&self) -> bool {
        *self == P2CA_A::CA0_CA7
    }
    #[doc = "Connect + to CA1, no connection to -"]
    #[inline(always)]
    pub fn is_ca1_none(&self) -> bool {
        *self == P2CA_A::CA1_NONE
    }
    #[doc = "Connect + to CA2, no connection to -"]
    #[inline(always)]
    pub fn is_ca2_none(&self) -> bool {
        *self == P2CA_A::CA2_NONE
    }
    #[doc = "Connect + to CA1, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca1_ca1(&self) -> bool {
        *self == P2CA_A::CA1_CA1
    }
    #[doc = "Connect + to CA2, connect - to CA1"]
    #[inline(always)]
    pub fn is_ca2_ca1(&self) -> bool {
        *self == P2CA_A::CA2_CA1
    }
    #[doc = "Connect + to CA1, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca1_ca2(&self) -> bool {
        *self == P2CA_A::CA1_CA2
    }
    #[doc = "Connect + to CA2, connect - to CA2"]
    #[inline(always)]
    pub fn is_ca2_ca2(&self) -> bool {
        *self == P2CA_A::CA2_CA2
    }
    #[doc = "Connect + to CA1, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca1_ca3(&self) -> bool {
        *self == P2CA_A::CA1_CA3
    }
    #[doc = "Connect + to CA2, connect - to CA3"]
    #[inline(always)]
    pub fn is_ca2_ca3(&self) -> bool {
        *self == P2CA_A::CA2_CA3
    }
    #[doc = "Connect + to CA1, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca1_ca4(&self) -> bool {
        *self == P2CA_A::CA1_CA4
    }
    #[doc = "Connect + to CA2, connect - to CA4"]
    #[inline(always)]
    pub fn is_ca2_ca4(&self) -> bool {
        *self == P2CA_A::CA2_CA4
    }
    #[doc = "Connect + to CA1, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca1_ca5(&self) -> bool {
        *self == P2CA_A::CA1_CA5
    }
    #[doc = "Connect + to CA2, connect - to CA5"]
    #[inline(always)]
    pub fn is_ca2_ca5(&self) -> bool {
        *self == P2CA_A::CA2_CA5
    }
    #[doc = "Connect + to CA1, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca1_ca6(&self) -> bool {
        *self == P2CA_A::CA1_CA6
    }
    #[doc = "Connect + to CA2, connect - to CA6"]
    #[inline(always)]
    pub fn is_ca2_ca6(&self) -> bool {
        *self == P2CA_A::CA2_CA6
    }
    #[doc = "Connect + to CA1, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca1_ca7(&self) -> bool {
        *self == P2CA_A::CA1_CA7
    }
    #[doc = "Connect + to CA2, connect - to CA7"]
    #[inline(always)]
    pub fn is_ca2_ca7(&self) -> bool {
        *self == P2CA_A::CA2_CA7
    }
}
#[doc = "Field `P2CA` writer - Comp. A +Terminal Multiplexer"]
pub type P2CA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, P2CA_A, crate::Safe>;
impl<'a, REG> P2CA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No + or - connection"]
    #[inline(always)]
    pub fn none_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_NONE)
    }
    #[doc = "Connect + to CA0, no connection to -"]
    #[inline(always)]
    pub fn ca0_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_NONE)
    }
    #[doc = "No connection to +, connect - to CA1"]
    #[inline(always)]
    pub fn none_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA1)
    }
    #[doc = "Connect + to CA0, connect - to CA1"]
    #[inline(always)]
    pub fn ca0_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA1)
    }
    #[doc = "No connection to +, connect - to CA2"]
    #[inline(always)]
    pub fn none_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA2)
    }
    #[doc = "Connect + to CA0, connect - to CA2"]
    #[inline(always)]
    pub fn ca0_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA2)
    }
    #[doc = "No connection to +, connect - to CA3"]
    #[inline(always)]
    pub fn none_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA3)
    }
    #[doc = "Connect + to CA0, connect - to CA3"]
    #[inline(always)]
    pub fn ca0_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA3)
    }
    #[doc = "No connection to +, connect - to CA4"]
    #[inline(always)]
    pub fn none_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA4)
    }
    #[doc = "Connect + to CA0, connect - to CA4"]
    #[inline(always)]
    pub fn ca0_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA4)
    }
    #[doc = "No connection to +, connect - to CA5"]
    #[inline(always)]
    pub fn none_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA5)
    }
    #[doc = "Connect + to CA0, connect - to CA5"]
    #[inline(always)]
    pub fn ca0_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA5)
    }
    #[doc = "No connection to +, connect - to CA6"]
    #[inline(always)]
    pub fn none_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA6)
    }
    #[doc = "Connect + to CA0, connect - to CA6"]
    #[inline(always)]
    pub fn ca0_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA6)
    }
    #[doc = "No connection to +, connect - to CA7"]
    #[inline(always)]
    pub fn none_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::NONE_CA7)
    }
    #[doc = "Connect + to CA0, connect - to CA7"]
    #[inline(always)]
    pub fn ca0_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA0_CA7)
    }
    #[doc = "Connect + to CA1, no connection to -"]
    #[inline(always)]
    pub fn ca1_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_NONE)
    }
    #[doc = "Connect + to CA2, no connection to -"]
    #[inline(always)]
    pub fn ca2_none(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_NONE)
    }
    #[doc = "Connect + to CA1, connect - to CA1"]
    #[inline(always)]
    pub fn ca1_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA1)
    }
    #[doc = "Connect + to CA2, connect - to CA1"]
    #[inline(always)]
    pub fn ca2_ca1(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA1)
    }
    #[doc = "Connect + to CA1, connect - to CA2"]
    #[inline(always)]
    pub fn ca1_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA2)
    }
    #[doc = "Connect + to CA2, connect - to CA2"]
    #[inline(always)]
    pub fn ca2_ca2(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA2)
    }
    #[doc = "Connect + to CA1, connect - to CA3"]
    #[inline(always)]
    pub fn ca1_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA3)
    }
    #[doc = "Connect + to CA2, connect - to CA3"]
    #[inline(always)]
    pub fn ca2_ca3(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA3)
    }
    #[doc = "Connect + to CA1, connect - to CA4"]
    #[inline(always)]
    pub fn ca1_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA4)
    }
    #[doc = "Connect + to CA2, connect - to CA4"]
    #[inline(always)]
    pub fn ca2_ca4(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA4)
    }
    #[doc = "Connect + to CA1, connect - to CA5"]
    #[inline(always)]
    pub fn ca1_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA5)
    }
    #[doc = "Connect + to CA2, connect - to CA5"]
    #[inline(always)]
    pub fn ca2_ca5(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA5)
    }
    #[doc = "Connect + to CA1, connect - to CA6"]
    #[inline(always)]
    pub fn ca1_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA6)
    }
    #[doc = "Connect + to CA2, connect - to CA6"]
    #[inline(always)]
    pub fn ca2_ca6(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA6)
    }
    #[doc = "Connect + to CA1, connect - to CA7"]
    #[inline(always)]
    pub fn ca1_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA1_CA7)
    }
    #[doc = "Connect + to CA2, connect - to CA7"]
    #[inline(always)]
    pub fn ca2_ca7(self) -> &'a mut crate::W<REG> {
        self.variant(P2CA_A::CA2_CA7)
    }
}
#[doc = "Field `CASHORT` reader - Comp. A Short + and - Terminals"]
pub type CASHORT_R = crate::BitReader;
#[doc = "Field `CASHORT` writer - Comp. A Short + and - Terminals"]
pub type CASHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&self) -> CAOUT_R {
        CAOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&self) -> P2CA_R {
        P2CA_R::new((self.bits >> 2) & 0x1f)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&self) -> CASHORT_R {
        CASHORT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comp. A Output"]
    #[inline(always)]
    pub fn caout(&mut self) -> CAOUT_W<CACTL2_SPEC> {
        CAOUT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comp. A Enable Output Filter"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W<CACTL2_SPEC> {
        CAF_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Comp. A +Terminal Multiplexer"]
    #[inline(always)]
    pub fn p2ca(&mut self) -> P2CA_W<CACTL2_SPEC> {
        P2CA_W::new(self, 2)
    }
    #[doc = "Bit 7 - Comp. A Short + and - Terminals"]
    #[inline(always)]
    pub fn cashort(&mut self) -> CASHORT_W<CACTL2_SPEC> {
        CASHORT_W::new(self, 7)
    }
}
#[doc = "Comparator A Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cactl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cactl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACTL2_SPEC;
impl crate::RegisterSpec for CACTL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cactl2::R`](R) reader structure"]
impl crate::Readable for CACTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cactl2::W`](W) writer structure"]
impl crate::Writable for CACTL2_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CACTL2 to value 0"]
impl crate::Resettable for CACTL2_SPEC {
    const RESET_VALUE: u8 = 0;
}
