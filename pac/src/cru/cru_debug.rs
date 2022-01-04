#[doc = "Register `CRU_DEBUG` reader"]
pub struct R(crate::R<CRU_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRU_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRU_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRU_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRU_DEBUG` writer"]
pub struct W(crate::W<CRU_DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRU_DEBUG_SPEC>;
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
impl From<crate::W<CRU_DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRU_DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the clock to be monitored\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRU_DEBUG_SELECT_A {
    #[doc = "1: Select clock C00 as the clock to debug"]
    MONITOR_C00_CLOCK = 1,
    #[doc = "2: Select clock C01 as the clock to debug"]
    MONITOR_C01_CLOCK = 2,
    #[doc = "3: Select clock C02 as the clock to debug"]
    MONITOR_C02_CLOCK = 3,
    #[doc = "4: Select clock C08X4 as the clock to debug"]
    MONITOR_C08X4_CLOCK = 4,
    #[doc = "5: Select clock C08X1 as the clock to debug"]
    MONITOR_C08X1_CLOCK = 5,
    #[doc = "6: Select clock C09 as the clock to debug"]
    MONITOR_C09_CLOCK = 6,
    #[doc = "7: Select clock C10 as the clock to debug"]
    MONITOR_C10_CLOCK = 7,
    #[doc = "8: Select clock C11 as the clock to debug"]
    MONITOR_C11_CLOCK = 8,
    #[doc = "9: Select clock CS as the clock to debug"]
    MONITOR_CS_CLOCK = 9,
    #[doc = "10: Select clock C16 as the clock to debug"]
    MONITOR_C16_CLOCK = 10,
    #[doc = "11: Select clock C19 as the clock to debug"]
    MONITOR_C19_CLOCK = 11,
    #[doc = "12: Select clock C20/C32 as the clock to debug"]
    MONITOR_C20_C32_CLOCK = 12,
    #[doc = "13: Select clock C21 as the clock to debug"]
    MONITOR_C21_CLOCK = 13,
    #[doc = "14: Select clock C23 as the clock to debug"]
    MONITOR_C23_CLOCK = 14,
    #[doc = "15: Select clock C30/C31 as the clock to debug"]
    MONITOR_C30_C31_CLOCK = 15,
}
impl From<CRU_DEBUG_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CRU_DEBUG_SELECT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRU_DEBUG_Select` reader - Select the clock to be monitored"]
pub struct CRU_DEBUG_SELECT_R(crate::FieldReader<u8, CRU_DEBUG_SELECT_A>);
impl CRU_DEBUG_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRU_DEBUG_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CRU_DEBUG_SELECT_A> {
        match self.bits {
            1 => Some(CRU_DEBUG_SELECT_A::MONITOR_C00_CLOCK),
            2 => Some(CRU_DEBUG_SELECT_A::MONITOR_C01_CLOCK),
            3 => Some(CRU_DEBUG_SELECT_A::MONITOR_C02_CLOCK),
            4 => Some(CRU_DEBUG_SELECT_A::MONITOR_C08X4_CLOCK),
            5 => Some(CRU_DEBUG_SELECT_A::MONITOR_C08X1_CLOCK),
            6 => Some(CRU_DEBUG_SELECT_A::MONITOR_C09_CLOCK),
            7 => Some(CRU_DEBUG_SELECT_A::MONITOR_C10_CLOCK),
            8 => Some(CRU_DEBUG_SELECT_A::MONITOR_C11_CLOCK),
            9 => Some(CRU_DEBUG_SELECT_A::MONITOR_CS_CLOCK),
            10 => Some(CRU_DEBUG_SELECT_A::MONITOR_C16_CLOCK),
            11 => Some(CRU_DEBUG_SELECT_A::MONITOR_C19_CLOCK),
            12 => Some(CRU_DEBUG_SELECT_A::MONITOR_C20_C32_CLOCK),
            13 => Some(CRU_DEBUG_SELECT_A::MONITOR_C21_CLOCK),
            14 => Some(CRU_DEBUG_SELECT_A::MONITOR_C23_CLOCK),
            15 => Some(CRU_DEBUG_SELECT_A::MONITOR_C30_C31_CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MONITOR_C00_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c00_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C00_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C01_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c01_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C01_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C02_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c02_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C02_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C08X4_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c08x4_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C08X4_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C08X1_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c08x1_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C08X1_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C09_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c09_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C09_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C10_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c10_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C10_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C11_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c11_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C11_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_CS_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_cs_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_CS_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C16_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c16_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C16_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C19_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c19_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C19_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C20_C32_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c20_c32_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C20_C32_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C21_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c21_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C21_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C23_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c23_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C23_CLOCK
    }
    #[doc = "Checks if the value of the field is `MONITOR_C30_C31_CLOCK`"]
    #[inline(always)]
    pub fn is_monitor_c30_c31_clock(&self) -> bool {
        **self == CRU_DEBUG_SELECT_A::MONITOR_C30_C31_CLOCK
    }
}
impl core::ops::Deref for CRU_DEBUG_SELECT_R {
    type Target = crate::FieldReader<u8, CRU_DEBUG_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRU_DEBUG_Select` writer - Select the clock to be monitored"]
pub struct CRU_DEBUG_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRU_DEBUG_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRU_DEBUG_SELECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select clock C00 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c00_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C00_CLOCK)
    }
    #[doc = "Select clock C01 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c01_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C01_CLOCK)
    }
    #[doc = "Select clock C02 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c02_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C02_CLOCK)
    }
    #[doc = "Select clock C08X4 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c08x4_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C08X4_CLOCK)
    }
    #[doc = "Select clock C08X1 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c08x1_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C08X1_CLOCK)
    }
    #[doc = "Select clock C09 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c09_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C09_CLOCK)
    }
    #[doc = "Select clock C10 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c10_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C10_CLOCK)
    }
    #[doc = "Select clock C11 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c11_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C11_CLOCK)
    }
    #[doc = "Select clock CS as the clock to debug"]
    #[inline(always)]
    pub fn monitor_cs_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_CS_CLOCK)
    }
    #[doc = "Select clock C16 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c16_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C16_CLOCK)
    }
    #[doc = "Select clock C19 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c19_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C19_CLOCK)
    }
    #[doc = "Select clock C20/C32 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c20_c32_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C20_C32_CLOCK)
    }
    #[doc = "Select clock C21 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c21_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C21_CLOCK)
    }
    #[doc = "Select clock C23 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c23_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C23_CLOCK)
    }
    #[doc = "Select clock C30/C31 as the clock to debug"]
    #[inline(always)]
    pub fn monitor_c30_c31_clock(self) -> &'a mut W {
        self.variant(CRU_DEBUG_SELECT_A::MONITOR_C30_C31_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Select the clock to be monitored"]
    #[inline(always)]
    pub fn cru_debug_select(&self) -> CRU_DEBUG_SELECT_R {
        CRU_DEBUG_SELECT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select the clock to be monitored"]
    #[inline(always)]
    pub fn cru_debug_select(&mut self) -> CRU_DEBUG_SELECT_W {
        CRU_DEBUG_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRU Debug registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cru_debug](index.html) module"]
pub struct CRU_DEBUG_SPEC;
impl crate::RegisterSpec for CRU_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cru_debug::R](R) reader structure"]
impl crate::Readable for CRU_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cru_debug::W](W) writer structure"]
impl crate::Writable for CRU_DEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRU_DEBUG to value 0"]
impl crate::Resettable for CRU_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
