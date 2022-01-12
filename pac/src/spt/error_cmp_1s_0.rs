#[doc = "Register `ERROR_CMP_1S_0` reader"]
pub struct R(crate::R<ERROR_CMP_1S_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_1S_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_1S_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_1S_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_1S_0` writer"]
pub struct W(crate::W<ERROR_CMP_1S_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_1S_0_SPEC>;
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
impl From<crate::W<ERROR_CMP_1S_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_1S_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR_CMP_1S_0` reader - Each 1s period contains sub-events for every 40ms counted, and a subevent for the first ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the subevent is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 1ms event, bit 3:2 = 1st 40ms event ... 31:30 = 15th 40ms event\\]"]
pub struct ERROR_CMP_1S_0_R(crate::FieldReader<u32, u32>);
impl ERROR_CMP_1S_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERROR_CMP_1S_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_CMP_1S_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_CMP_1S_0` writer - Each 1s period contains sub-events for every 40ms counted, and a subevent for the first ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the subevent is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 1ms event, bit 3:2 = 1st 40ms event ... 31:30 = 15th 40ms event\\]"]
pub struct ERROR_CMP_1S_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CMP_1S_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Each 1s period contains sub-events for every 40ms counted, and a subevent for the first ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the subevent is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 1ms event, bit 3:2 = 1st 40ms event ... 31:30 = 15th 40ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_0(&self) -> ERROR_CMP_1S_0_R {
        ERROR_CMP_1S_0_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each 1s period contains sub-events for every 40ms counted, and a subevent for the first ms counted. Each bit-pair in this field corresponds to one of those sub-events. Values of 0 or 2 in bit pairs do nothing. A value of 3 means the counter should decrease 1ms everytime the subevent is triggered, and a value of 1 means the counter should increment 1ms every time the event is triggered. \\[bits 1:0 = 1st 1ms event, bit 3:2 = 1st 40ms event ... 31:30 = 15th 40ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_1s_0(&mut self) -> ERROR_CMP_1S_0_W {
        ERROR_CMP_1S_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1 sec Increment Error Compensation 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_1s_0](index.html) module"]
pub struct ERROR_CMP_1S_0_SPEC;
impl crate::RegisterSpec for ERROR_CMP_1S_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_1s_0::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_1S_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_1s_0::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_1S_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_1S_0 to value 0"]
impl crate::Resettable for ERROR_CMP_1S_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
