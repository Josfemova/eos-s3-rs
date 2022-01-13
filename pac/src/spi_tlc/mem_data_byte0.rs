#[doc = "Register `MemDataByte0` reader"]
pub struct R(crate::R<MEMDATABYTE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMDATABYTE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMDATABYTE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMDATABYTE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemDataByte0` writer"]
pub struct W(crate::W<MEMDATABYTE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMDATABYTE0_SPEC>;
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
impl From<crate::W<MEMDATABYTE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMDATABYTE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemDataByte0` reader - First Byte (LSB) of AHB memory data"]
pub struct MEMDATABYTE0_R(crate::FieldReader<u8, u8>);
impl MEMDATABYTE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMDATABYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMDATABYTE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemDataByte0` writer - First Byte (LSB) of AHB memory data"]
pub struct MEMDATABYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMDATABYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - First Byte (LSB) of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte0(&self) -> MEMDATABYTE0_R {
        MEMDATABYTE0_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Byte (LSB) of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte0(&mut self) -> MEMDATABYTE0_W {
        MEMDATABYTE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First Byte (LSB) of AHB memory data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_data_byte0](index.html) module"]
pub struct MEMDATABYTE0_SPEC;
impl crate::RegisterSpec for MEMDATABYTE0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_data_byte0::R](R) reader structure"]
impl crate::Readable for MEMDATABYTE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_data_byte0::W](W) writer structure"]
impl crate::Writable for MEMDATABYTE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemDataByte0 to value 0"]
impl crate::Resettable for MEMDATABYTE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
