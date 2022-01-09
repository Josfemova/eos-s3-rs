#[doc = "Register `FFE0_BREAK_POINT_CFG` reader"]
pub struct R(crate::R<FFE0_BREAK_POINT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE0_BREAK_POINT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE0_BREAK_POINT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE0_BREAK_POINT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE0_BREAK_POINT_CFG` writer"]
pub struct W(crate::W<FFE0_BREAK_POINT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE0_BREAK_POINT_CFG_SPEC>;
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
impl From<crate::W<FFE0_BREAK_POINT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE0_BREAK_POINT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFE0_BP_EN` reader - break point execution -- 0 : Disabled -- 1 : Enabled"]
pub struct FFE0_BP_EN_R(crate::FieldReader<bool, bool>);
impl FFE0_BP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BP_EN` writer - break point execution -- 0 : Disabled -- 1 : Enabled"]
pub struct FFE0_BP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BP_EN_W<'a> {
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
#[doc = "Field `FFE0_FORCE_STOP` reader - This causes the FFE to immediately halt execution."]
pub struct FFE0_FORCE_STOP_R(crate::FieldReader<bool, bool>);
impl FFE0_FORCE_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_FORCE_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_FORCE_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_FORCE_STOP` writer - This causes the FFE to immediately halt execution."]
pub struct FFE0_FORCE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_FORCE_STOP_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FFE0_BreakPt_Sw-Brk` reader - Causes the signal to toggle when written with a '1' in this bit position."]
pub struct FFE0_BREAKPT_SWBRK_R(crate::FieldReader<bool, bool>);
impl FFE0_BREAKPT_SWBRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BREAKPT_SWBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BREAKPT_SWBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BreakPt_Sw-Brk` writer - Causes the signal to toggle when written with a '1' in this bit position."]
pub struct FFE0_BREAKPT_SWBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BREAKPT_SWBRK_W<'a> {
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
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - break point execution -- 0 : Disabled -- 1 : Enabled"]
    #[inline(always)]
    pub fn ffe0_bp_en(&self) -> FFE0_BP_EN_R {
        FFE0_BP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This causes the FFE to immediately halt execution."]
    #[inline(always)]
    pub fn ffe0_force_stop(&self) -> FFE0_FORCE_STOP_R {
        FFE0_FORCE_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Causes the signal to toggle when written with a '1' in this bit position."]
    #[inline(always)]
    pub fn ffe0_break_pt_sw_brk(&self) -> FFE0_BREAKPT_SWBRK_R {
        FFE0_BREAKPT_SWBRK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - break point execution -- 0 : Disabled -- 1 : Enabled"]
    #[inline(always)]
    pub fn ffe0_bp_en(&mut self) -> FFE0_BP_EN_W {
        FFE0_BP_EN_W { w: self }
    }
    #[doc = "Bit 1 - This causes the FFE to immediately halt execution."]
    #[inline(always)]
    pub fn ffe0_force_stop(&mut self) -> FFE0_FORCE_STOP_W {
        FFE0_FORCE_STOP_W { w: self }
    }
    #[doc = "Bit 1 - Causes the signal to toggle when written with a '1' in this bit position."]
    #[inline(always)]
    pub fn ffe0_break_pt_sw_brk(&mut self) -> FFE0_BREAKPT_SWBRK_W {
        FFE0_BREAKPT_SWBRK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Break point control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe0_break_point_cfg](index.html) module"]
pub struct FFE0_BREAK_POINT_CFG_SPEC;
impl crate::RegisterSpec for FFE0_BREAK_POINT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe0_break_point_cfg::R](R) reader structure"]
impl crate::Readable for FFE0_BREAK_POINT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe0_break_point_cfg::W](W) writer structure"]
impl crate::Writable for FFE0_BREAK_POINT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE0_BREAK_POINT_CFG to value 0"]
impl crate::Resettable for FFE0_BREAK_POINT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
