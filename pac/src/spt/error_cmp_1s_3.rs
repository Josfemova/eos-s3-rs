#[doc = "Register `ERROR_CMP_1S_3` reader"]
pub struct R(crate::R<ERROR_CMP_1S_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_1S_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_1S_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_1S_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_1S_3` writer"]
pub struct W(crate::W<ERROR_CMP_1S_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_1S_3_SPEC>;
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
impl From<crate::W<ERROR_CMP_1S_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_1S_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR_CMP_1S_3` reader - Each 1s period contains sub-events for every 150ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 150ms event, bit 3:2 = 2nd 150ms event ... 11:10 = 6th 150ms event\\]"]
pub struct ERROR_CMP_1S_3_R(crate::FieldReader<u16, u16>);
impl ERROR_CMP_1S_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ERROR_CMP_1S_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_CMP_1S_3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_CMP_1S_3` writer - Each 1s period contains sub-events for every 150ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 150ms event, bit 3:2 = 2nd 150ms event ... 11:10 = 6th 150ms event\\]"]
pub struct ERROR_CMP_1S_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CMP_1S_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Each 1s period contains sub-events for every 150ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 150ms event, bit 3:2 = 2nd 150ms event ... 11:10 = 6th 150ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_3(&self) -> ERROR_CMP_1S_3_R {
        ERROR_CMP_1S_3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Each 1s period contains sub-events for every 150ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 150ms event, bit 3:2 = 2nd 150ms event ... 11:10 = 6th 150ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_3(&mut self) -> ERROR_CMP_1S_3_W {
        ERROR_CMP_1S_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1 sec Incremente Error Compensation 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_1s_3](index.html) module"]
pub struct ERROR_CMP_1S_3_SPEC;
impl crate::RegisterSpec for ERROR_CMP_1S_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_1s_3::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_1S_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_1s_3::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_1S_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_1S_3 to value 0"]
impl crate::Resettable for ERROR_CMP_1S_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
