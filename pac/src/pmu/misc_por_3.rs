#[doc = "Register `MISC_POR_3` reader"]
pub struct R(crate::R<MISC_POR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_POR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_POR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_POR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_POR_3` writer"]
pub struct W(crate::W<MISC_POR_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_POR_3_SPEC>;
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
impl From<crate::W<MISC_POR_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_POR_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Scratch_byte_0` reader - Scratch Bytes, General Purpose Registers"]
pub struct SCRATCH_BYTE_0_R(crate::FieldReader<u8, u8>);
impl SCRATCH_BYTE_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCRATCH_BYTE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH_BYTE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Scratch_byte_0` writer - Scratch Bytes, General Purpose Registers"]
pub struct SCRATCH_BYTE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH_BYTE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `Scratch_byte_1` reader - Scratch Bytes, General Purpose Registers"]
pub struct SCRATCH_BYTE_1_R(crate::FieldReader<u8, u8>);
impl SCRATCH_BYTE_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCRATCH_BYTE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRATCH_BYTE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Scratch_byte_1` writer - Scratch Bytes, General Purpose Registers"]
pub struct SCRATCH_BYTE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH_BYTE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Scratch Bytes, General Purpose Registers"]
    #[inline(always)]
    pub fn scratch_byte_0(&self) -> SCRATCH_BYTE_0_R {
        SCRATCH_BYTE_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Scratch Bytes, General Purpose Registers"]
    #[inline(always)]
    pub fn scratch_byte_1(&self) -> SCRATCH_BYTE_1_R {
        SCRATCH_BYTE_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scratch Bytes, General Purpose Registers"]
    #[inline(always)]
    pub fn scratch_byte_0(&mut self) -> SCRATCH_BYTE_0_W {
        SCRATCH_BYTE_0_W { w: self }
    }
    #[doc = "Bits 8:15 - Scratch Bytes, General Purpose Registers"]
    #[inline(always)]
    pub fn scratch_byte_1(&mut self) -> SCRATCH_BYTE_1_W {
        SCRATCH_BYTE_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On POR Reset Domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_por_3](index.html) module"]
pub struct MISC_POR_3_SPEC;
impl crate::RegisterSpec for MISC_POR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_por_3::R](R) reader structure"]
impl crate::Readable for MISC_POR_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_por_3::W](W) writer structure"]
impl crate::Writable for MISC_POR_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_POR_3 to value 0"]
impl crate::Resettable for MISC_POR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
