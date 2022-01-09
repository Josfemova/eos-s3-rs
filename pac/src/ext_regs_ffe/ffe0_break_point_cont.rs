#[doc = "Register `FFE0_BREAK_POINT_CONT` reader"]
pub struct R(crate::R<FFE0_BREAK_POINT_CONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE0_BREAK_POINT_CONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE0_BREAK_POINT_CONT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE0_BREAK_POINT_CONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE0_BREAK_POINT_CONT` writer"]
pub struct W(crate::W<FFE0_BREAK_POINT_CONT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE0_BREAK_POINT_CONT_SPEC>;
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
impl From<crate::W<FFE0_BREAK_POINT_CONT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE0_BREAK_POINT_CONT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFE0_BP_CONT` reader - This restarts FFE code execution following a pause due to reaching a 'break point'."]
pub struct FFE0_BP_CONT_R(crate::FieldReader<bool, bool>);
impl FFE0_BP_CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_BP_CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_BP_CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_BP_CONT` writer - This restarts FFE code execution following a pause due to reaching a 'break point'."]
pub struct FFE0_BP_CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_BP_CONT_W<'a> {
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
#[doc = "Field `SM0_BP_CONT` reader - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
pub struct SM0_BP_CONT_R(crate::FieldReader<bool, bool>);
impl SM0_BP_CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_BP_CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_BP_CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_BP_CONT` writer - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
pub struct SM0_BP_CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_BP_CONT_W<'a> {
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
#[doc = "Field `SM1_BP_CONT` reader - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
pub struct SM1_BP_CONT_R(crate::FieldReader<bool, bool>);
impl SM1_BP_CONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_BP_CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_BP_CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_BP_CONT` writer - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
pub struct SM1_BP_CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_BP_CONT_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This restarts FFE code execution following a pause due to reaching a 'break point'."]
    #[inline(always)]
    pub fn ffe0_bp_cont(&self) -> FFE0_BP_CONT_R {
        FFE0_BP_CONT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
    #[inline(always)]
    pub fn sm0_bp_cont(&self) -> SM0_BP_CONT_R {
        SM0_BP_CONT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
    #[inline(always)]
    pub fn sm1_bp_cont(&self) -> SM1_BP_CONT_R {
        SM1_BP_CONT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This restarts FFE code execution following a pause due to reaching a 'break point'."]
    #[inline(always)]
    pub fn ffe0_bp_cont(&mut self) -> FFE0_BP_CONT_W {
        FFE0_BP_CONT_W { w: self }
    }
    #[doc = "Bit 1 - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
    #[inline(always)]
    pub fn sm0_bp_cont(&mut self) -> SM0_BP_CONT_W {
        SM0_BP_CONT_W { w: self }
    }
    #[doc = "Bit 2 - This is a single, host controlled input toggle signal, Break Point Match Continue. Software uses this toggle signal to resume code execution from the Break Point condition."]
    #[inline(always)]
    pub fn sm1_bp_cont(&mut self) -> SM1_BP_CONT_W {
        SM1_BP_CONT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Seems to be another breakpoint control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe0_break_point_cont](index.html) module"]
pub struct FFE0_BREAK_POINT_CONT_SPEC;
impl crate::RegisterSpec for FFE0_BREAK_POINT_CONT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe0_break_point_cont::R](R) reader structure"]
impl crate::Readable for FFE0_BREAK_POINT_CONT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe0_break_point_cont::W](W) writer structure"]
impl crate::Writable for FFE0_BREAK_POINT_CONT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE0_BREAK_POINT_CONT to value 0"]
impl crate::Resettable for FFE0_BREAK_POINT_CONT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
