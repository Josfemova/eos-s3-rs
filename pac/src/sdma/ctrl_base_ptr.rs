#[doc = "Register `CTRL_BASE_PTR` reader"]
pub struct R(crate::R<CTRL_BASE_PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_BASE_PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_BASE_PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_BASE_PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_BASE_PTR` writer"]
pub struct W(crate::W<CTRL_BASE_PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_BASE_PTR_SPEC>;
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
impl From<crate::W<CTRL_BASE_PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_BASE_PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ctrl_base_ptr` reader - Pointer to the base address of the primary data structure"]
pub struct CTRL_BASE_PTR_R(crate::FieldReader<u32, u32>);
impl CTRL_BASE_PTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CTRL_BASE_PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_BASE_PTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ctrl_base_ptr` writer - Pointer to the base address of the primary data structure"]
pub struct CTRL_BASE_PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_BASE_PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9))
            | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - Pointer to the base address of the primary data structure"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&self) -> CTRL_BASE_PTR_R {
        CTRL_BASE_PTR_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 9:31 - Pointer to the base address of the primary data structure"]
    #[inline(always)]
    pub fn ctrl_base_ptr(&mut self) -> CTRL_BASE_PTR_W {
        CTRL_BASE_PTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control the pointer to the base address of the primary data structure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_base_ptr](index.html) module"]
pub struct CTRL_BASE_PTR_SPEC;
impl crate::RegisterSpec for CTRL_BASE_PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_base_ptr::R](R) reader structure"]
impl crate::Readable for CTRL_BASE_PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_base_ptr::W](W) writer structure"]
impl crate::Writable for CTRL_BASE_PTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_BASE_PTR to value 0"]
impl crate::Resettable for CTRL_BASE_PTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
