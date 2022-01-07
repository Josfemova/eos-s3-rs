#[doc = "Register `MISC_POR_0` reader"]
pub struct R(crate::R<MISC_POR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_POR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_POR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_POR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_POR_0` writer"]
pub struct W(crate::W<MISC_POR_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_POR_0_SPEC>;
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
impl From<crate::W<MISC_POR_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_POR_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RWHC: In AP mode, (SPIS_SS is High by default) ( In Wearable Mode, it has NO effect). The Hardware will clear this bit if `AP_Reboot_enable_N` is programmed and M4 is waking up from Mode 1 (SD) to ensure the M4 is kept on Reset once waking up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_RST_RELEASE_A {
    #[doc = "0: Keep M4 Core on Reset"]
    KEEP_ON_RESET = 0,
    #[doc = "1: Release M4 Core Reset"]
    RELEASE_FROM_RESET = 1,
}
impl From<M4_RST_RELEASE_A> for bool {
    #[inline(always)]
    fn from(variant: M4_RST_RELEASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M4_Rst_Release` reader - RWHC: In AP mode, (SPIS_SS is High by default) ( In Wearable Mode, it has NO effect). The Hardware will clear this bit if `AP_Reboot_enable_N` is programmed and M4 is waking up from Mode 1 (SD) to ensure the M4 is kept on Reset once waking up."]
pub struct M4_RST_RELEASE_R(crate::FieldReader<bool, M4_RST_RELEASE_A>);
impl M4_RST_RELEASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4_RST_RELEASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4_RST_RELEASE_A {
        match self.bits {
            false => M4_RST_RELEASE_A::KEEP_ON_RESET,
            true => M4_RST_RELEASE_A::RELEASE_FROM_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP_ON_RESET`"]
    #[inline(always)]
    pub fn is_keep_on_reset(&self) -> bool {
        **self == M4_RST_RELEASE_A::KEEP_ON_RESET
    }
    #[doc = "Checks if the value of the field is `RELEASE_FROM_RESET`"]
    #[inline(always)]
    pub fn is_release_from_reset(&self) -> bool {
        **self == M4_RST_RELEASE_A::RELEASE_FROM_RESET
    }
}
impl core::ops::Deref for M4_RST_RELEASE_R {
    type Target = crate::FieldReader<bool, M4_RST_RELEASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4_Rst_Release` writer - RWHC: In AP mode, (SPIS_SS is High by default) ( In Wearable Mode, it has NO effect). The Hardware will clear this bit if `AP_Reboot_enable_N` is programmed and M4 is waking up from Mode 1 (SD) to ensure the M4 is kept on Reset once waking up."]
pub struct M4_RST_RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_RST_RELEASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4_RST_RELEASE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Keep M4 Core on Reset"]
    #[inline(always)]
    pub fn keep_on_reset(self) -> &'a mut W {
        self.variant(M4_RST_RELEASE_A::KEEP_ON_RESET)
    }
    #[doc = "Release M4 Core Reset"]
    #[inline(always)]
    pub fn release_from_reset(self) -> &'a mut W {
        self.variant(M4_RST_RELEASE_A::RELEASE_FROM_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RWHC: In AP mode, (SPIS_SS is High by default) ( In Wearable Mode, it has NO effect). The Hardware will clear this bit if `AP_Reboot_enable_N` is programmed and M4 is waking up from Mode 1 (SD) to ensure the M4 is kept on Reset once waking up."]
    #[inline(always)]
    pub fn m4_rst_release(&self) -> M4_RST_RELEASE_R {
        M4_RST_RELEASE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWHC: In AP mode, (SPIS_SS is High by default) ( In Wearable Mode, it has NO effect). The Hardware will clear this bit if `AP_Reboot_enable_N` is programmed and M4 is waking up from Mode 1 (SD) to ensure the M4 is kept on Reset once waking up."]
    #[inline(always)]
    pub fn m4_rst_release(&mut self) -> M4_RST_RELEASE_W {
        M4_RST_RELEASE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On POR Reset Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_por_0](index.html) module"]
pub struct MISC_POR_0_SPEC;
impl crate::RegisterSpec for MISC_POR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_por_0::R](R) reader structure"]
impl crate::Readable for MISC_POR_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_por_0::W](W) writer structure"]
impl crate::Writable for MISC_POR_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_POR_0 to value 0"]
impl crate::Resettable for MISC_POR_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
