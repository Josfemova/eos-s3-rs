#[doc = "Register `A0_PMU_DBG_MON_SEL` reader"]
pub struct R(crate::R<A0_PMU_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A0_PMU_DBG_MON_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A0_PMU_DBG_MON_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A0_PMU_DBG_MON_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A0_PMU_DBG_MON_SEL` writer"]
pub struct W(crate::W<A0_PMU_DBG_MON_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A0_PMU_DBG_MON_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<A0_PMU_DBG_MON_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A0_PMU_DBG_MON_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select data to be shown in the PMU debug monitor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum A0_PMU_DEBUG_MON_SEL_A {
    #[doc = "0: {M4STM_KickOff, M4STM_BUSY, 6'h0}"]
    SHOW_M4STM_KICKOFF_AND_BUSY_PLUS_0X000000 = 0,
    #[doc = "1: `1`"]
    SHOW_A1 = 1,
    #[doc = "2: `10`"]
    SHOW_M4 = 2,
    #[doc = "3: `11`"]
    SHOW_M4S0 = 3,
    #[doc = "4: `100`"]
    SHOW_AD5 = 4,
    #[doc = "5: `101`"]
    SHOW_AD4 = 5,
    #[doc = "6: `110`"]
    SHOW_AD3 = 6,
    #[doc = "7: `111`"]
    SHOW_AD2 = 7,
    #[doc = "8: `1000`"]
    SHOW_AD1 = 8,
    #[doc = "9: `1001`"]
    SHOW_AD0 = 9,
    #[doc = "10: {FBSTM_KickOff, FBSTM_BUSY,6'h0}"]
    SHOW_FBST_KICKOFF_AND_BUSY_PLUS_0X000000 = 10,
    #[doc = "11: `1011`"]
    SHOW_I2S = 11,
    #[doc = "13: `1101`"]
    SHOW_M4S15 = 13,
    #[doc = "14: `1110`"]
    SHOW_M4S11 = 14,
    #[doc = "15: `1111`"]
    SHOW_M4S7 = 15,
    #[doc = "16: `10000`"]
    SHOW_M4S3 = 16,
    #[doc = "17: `10001`"]
    SHOW_PF = 17,
    #[doc = "18: `10010`"]
    SHOW_SDMA = 18,
    #[doc = "19: `10011`"]
    SHOW_FFE = 19,
    #[doc = "20: `10100`"]
    SHOW_FB = 20,
}
impl From<A0_PMU_DEBUG_MON_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: A0_PMU_DEBUG_MON_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `A0_PMU_DEBUG_MON_SEL` reader - Select data to be shown in the PMU debug monitor"]
pub struct A0_PMU_DEBUG_MON_SEL_R(
    crate::FieldReader<u8, A0_PMU_DEBUG_MON_SEL_A>,
);
impl A0_PMU_DEBUG_MON_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        A0_PMU_DEBUG_MON_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<A0_PMU_DEBUG_MON_SEL_A> {
        match self.bits {
            0 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4STM_KICKOFF_AND_BUSY_PLUS_0X000000),
            1 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_A1),
            2 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4),
            3 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S0),
            4 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD5),
            5 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD4),
            6 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD3),
            7 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD2),
            8 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD1),
            9 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD0),
            10 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_FBST_KICKOFF_AND_BUSY_PLUS_0X000000),
            11 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_I2S),
            13 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S15),
            14 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S11),
            15 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S7),
            16 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S3),
            17 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_PF),
            18 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_SDMA),
            19 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_FFE),
            20 => Some(A0_PMU_DEBUG_MON_SEL_A::SHOW_FB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHOW_M4STM_KICKOFF_AND_BUSY_PLUS_0X000000`"]
    #[inline(always)]
    pub fn is_show_m4stm_kickoff_and_busy_plus_0x000000(&self) -> bool {
        **self
            == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4STM_KICKOFF_AND_BUSY_PLUS_0X000000
    }
    #[doc = "Checks if the value of the field is `SHOW_A1`"]
    #[inline(always)]
    pub fn is_show_a1(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_A1
    }
    #[doc = "Checks if the value of the field is `SHOW_M4`"]
    #[inline(always)]
    pub fn is_show_m4(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4
    }
    #[doc = "Checks if the value of the field is `SHOW_M4S0`"]
    #[inline(always)]
    pub fn is_show_m4s0(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S0
    }
    #[doc = "Checks if the value of the field is `SHOW_AD5`"]
    #[inline(always)]
    pub fn is_show_ad5(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD5
    }
    #[doc = "Checks if the value of the field is `SHOW_AD4`"]
    #[inline(always)]
    pub fn is_show_ad4(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD4
    }
    #[doc = "Checks if the value of the field is `SHOW_AD3`"]
    #[inline(always)]
    pub fn is_show_ad3(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD3
    }
    #[doc = "Checks if the value of the field is `SHOW_AD2`"]
    #[inline(always)]
    pub fn is_show_ad2(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD2
    }
    #[doc = "Checks if the value of the field is `SHOW_AD1`"]
    #[inline(always)]
    pub fn is_show_ad1(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD1
    }
    #[doc = "Checks if the value of the field is `SHOW_AD0`"]
    #[inline(always)]
    pub fn is_show_ad0(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_AD0
    }
    #[doc = "Checks if the value of the field is `SHOW_FBST_KICKOFF_AND_BUSY_PLUS_0X000000`"]
    #[inline(always)]
    pub fn is_show_fbst_kickoff_and_busy_plus_0x000000(&self) -> bool {
        **self
            == A0_PMU_DEBUG_MON_SEL_A::SHOW_FBST_KICKOFF_AND_BUSY_PLUS_0X000000
    }
    #[doc = "Checks if the value of the field is `SHOW_I2S`"]
    #[inline(always)]
    pub fn is_show_i2s(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_I2S
    }
    #[doc = "Checks if the value of the field is `SHOW_M4S15`"]
    #[inline(always)]
    pub fn is_show_m4s15(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S15
    }
    #[doc = "Checks if the value of the field is `SHOW_M4S11`"]
    #[inline(always)]
    pub fn is_show_m4s11(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S11
    }
    #[doc = "Checks if the value of the field is `SHOW_M4S7`"]
    #[inline(always)]
    pub fn is_show_m4s7(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S7
    }
    #[doc = "Checks if the value of the field is `SHOW_M4S3`"]
    #[inline(always)]
    pub fn is_show_m4s3(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S3
    }
    #[doc = "Checks if the value of the field is `SHOW_PF`"]
    #[inline(always)]
    pub fn is_show_pf(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_PF
    }
    #[doc = "Checks if the value of the field is `SHOW_SDMA`"]
    #[inline(always)]
    pub fn is_show_sdma(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_SDMA
    }
    #[doc = "Checks if the value of the field is `SHOW_FFE`"]
    #[inline(always)]
    pub fn is_show_ffe(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_FFE
    }
    #[doc = "Checks if the value of the field is `SHOW_FB`"]
    #[inline(always)]
    pub fn is_show_fb(&self) -> bool {
        **self == A0_PMU_DEBUG_MON_SEL_A::SHOW_FB
    }
}
impl core::ops::Deref for A0_PMU_DEBUG_MON_SEL_R {
    type Target = crate::FieldReader<u8, A0_PMU_DEBUG_MON_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A0_PMU_DEBUG_MON_SEL` writer - Select data to be shown in the PMU debug monitor"]
pub struct A0_PMU_DEBUG_MON_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> A0_PMU_DEBUG_MON_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: A0_PMU_DEBUG_MON_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "{M4STM_KickOff, M4STM_BUSY, 6'h0}"]
    #[inline(always)]
    pub fn show_m4stm_kickoff_and_busy_plus_0x000000(self) -> &'a mut W {
        self.variant(
            A0_PMU_DEBUG_MON_SEL_A::SHOW_M4STM_KICKOFF_AND_BUSY_PLUS_0X000000,
        )
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn show_a1(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_A1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn show_m4(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn show_m4s0(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S0)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn show_ad5(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD5)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn show_ad4(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD4)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn show_ad3(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD3)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn show_ad2(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn show_ad1(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn show_ad0(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_AD0)
    }
    #[doc = "{FBSTM_KickOff, FBSTM_BUSY,6'h0}"]
    #[inline(always)]
    pub fn show_fbst_kickoff_and_busy_plus_0x000000(self) -> &'a mut W {
        self.variant(
            A0_PMU_DEBUG_MON_SEL_A::SHOW_FBST_KICKOFF_AND_BUSY_PLUS_0X000000,
        )
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn show_i2s(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_I2S)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn show_m4s15(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn show_m4s11(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S11)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn show_m4s7(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S7)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn show_m4s3(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_M4S3)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn show_pf(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_PF)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn show_sdma(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_SDMA)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn show_ffe(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_FFE)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn show_fb(self) -> &'a mut W {
        self.variant(A0_PMU_DEBUG_MON_SEL_A::SHOW_FB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Select data to be shown in the PMU debug monitor"]
    #[inline(always)]
    pub fn a0_pmu_debug_mon_sel(&self) -> A0_PMU_DEBUG_MON_SEL_R {
        A0_PMU_DEBUG_MON_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Select data to be shown in the PMU debug monitor"]
    #[inline(always)]
    pub fn a0_pmu_debug_mon_sel(&mut self) -> A0_PMU_DEBUG_MON_SEL_W {
        A0_PMU_DEBUG_MON_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the data present in the PMU debug monitor. The output will be (Except 0 and 10), {Status0, Status1, ISO, RET, GateCLK_N, Mem_DS, MP_Gate, RP_Gate)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0_pmu_dbg_mon_sel](index.html) module"]
pub struct A0_PMU_DBG_MON_SEL_SPEC;
impl crate::RegisterSpec for A0_PMU_DBG_MON_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a0_pmu_dbg_mon_sel::R](R) reader structure"]
impl crate::Readable for A0_PMU_DBG_MON_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a0_pmu_dbg_mon_sel::W](W) writer structure"]
impl crate::Writable for A0_PMU_DBG_MON_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A0_PMU_DBG_MON_SEL to value 0"]
impl crate::Resettable for A0_PMU_DBG_MON_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
