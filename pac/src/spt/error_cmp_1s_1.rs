#[doc = "Register `ERROR_CMP_1S_1` reader"]
pub struct R(crate::R<ERROR_CMP_1S_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_1S_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_1S_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_1S_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_1S_1` writer"]
pub struct W(crate::W<ERROR_CMP_1S_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_1S_1_SPEC>;
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
impl From<crate::W<ERROR_CMP_1S_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_1S_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR_CMP_1S_1` reader - Each 1s period contains sub-events for every 40ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. Starts from the 16th 40ms event. \\[bits 1:0 = 16st 40ms event, bit 3:2 = 17st 40ms event ... 17:16 = 24th 40ms event\\]"]
pub struct ERROR_CMP_1S_1_R(crate::FieldReader<u32, u32>);
impl ERROR_CMP_1S_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERROR_CMP_1S_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_CMP_1S_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_CMP_1S_1` writer - Each 1s period contains sub-events for every 40ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. Starts from the 16th 40ms event. \\[bits 1:0 = 16st 40ms event, bit 3:2 = 17st 40ms event ... 17:16 = 24th 40ms event\\]"]
pub struct ERROR_CMP_1S_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CMP_1S_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Each 1s period contains sub-events for every 40ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. Starts from the 16th 40ms event. \\[bits 1:0 = 16st 40ms event, bit 3:2 = 17st 40ms event ... 17:16 = 24th 40ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_1(&self) -> ERROR_CMP_1S_1_R {
        ERROR_CMP_1S_1_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Each 1s period contains sub-events for every 40ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the sub-event is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. Starts from the 16th 40ms event. \\[bits 1:0 = 16st 40ms event, bit 3:2 = 17st 40ms event ... 17:16 = 24th 40ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_1(&mut self) -> ERROR_CMP_1S_1_W {
        ERROR_CMP_1S_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1 sec Incremente Error Compensation 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_1s_1](index.html) module"]
pub struct ERROR_CMP_1S_1_SPEC;
impl crate::RegisterSpec for ERROR_CMP_1S_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_1s_1::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_1S_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_1s_1::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_1S_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_1S_1 to value 0"]
impl crate::Resettable for ERROR_CMP_1S_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
