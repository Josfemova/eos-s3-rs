#[doc = "Register `ERROR_CMP_40M` reader"]
pub struct R(crate::R<ERROR_CMP_40M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CMP_40M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CMP_40M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CMP_40M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CMP_40M` writer"]
pub struct W(crate::W<ERROR_CMP_40M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CMP_40M_SPEC>;
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
impl From<crate::W<ERROR_CMP_40M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CMP_40M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR_CMP_40M` reader - Each 40ms period contains sub-events. For each sub-event, you can add 1ms when it triggers. Theres a sub-event every odd number, with each bit of this field corresponding to that event, so: \\[bit 0 = 1st ms event, bit 1 = 3rd ms event ... bit 19 = 39th ms event\\]"]
pub struct ERROR_CMP_40M_R(crate::FieldReader<u32, u32>);
impl ERROR_CMP_40M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ERROR_CMP_40M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_CMP_40M_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_CMP_40M` writer - Each 40ms period contains sub-events. For each sub-event, you can add 1ms when it triggers. Theres a sub-event every odd number, with each bit of this field corresponding to that event, so: \\[bit 0 = 1st ms event, bit 1 = 3rd ms event ... bit 19 = 39th ms event\\]"]
pub struct ERROR_CMP_40M_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_CMP_40M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Each 40ms period contains sub-events. For each sub-event, you can add 1ms when it triggers. Theres a sub-event every odd number, with each bit of this field corresponding to that event, so: \\[bit 0 = 1st ms event, bit 1 = 3rd ms event ... bit 19 = 39th ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_40m(&self) -> ERROR_CMP_40M_R {
        ERROR_CMP_40M_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Each 40ms period contains sub-events. For each sub-event, you can add 1ms when it triggers. Theres a sub-event every odd number, with each bit of this field corresponding to that event, so: \\[bit 0 = 1st ms event, bit 1 = 3rd ms event ... bit 19 = 39th ms event\\]"]
    #[inline(always)]
    pub fn error_cmp_40m(&mut self) -> ERROR_CMP_40M_W {
        ERROR_CMP_40M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "40 msec increment error compensation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_cmp_40m](index.html) module"]
pub struct ERROR_CMP_40M_SPEC;
impl crate::RegisterSpec for ERROR_CMP_40M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_cmp_40m::R](R) reader structure"]
impl crate::Readable for ERROR_CMP_40M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_cmp_40m::W](W) writer structure"]
impl crate::Writable for ERROR_CMP_40M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERROR_CMP_40M to value 0"]
impl crate::Resettable for ERROR_CMP_40M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
