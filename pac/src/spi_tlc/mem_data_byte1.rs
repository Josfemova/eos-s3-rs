#[doc = "Register `MemDataByte1` reader"]
pub struct R(crate::R<MEMDATABYTE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMDATABYTE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMDATABYTE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMDATABYTE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemDataByte1` writer"]
pub struct W(crate::W<MEMDATABYTE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMDATABYTE1_SPEC>;
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
impl From<crate::W<MEMDATABYTE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMDATABYTE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemDataByte1` reader - Second byte of AHB memory data"]
pub struct MEMDATABYTE1_R(crate::FieldReader<u8, u8>);
impl MEMDATABYTE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMDATABYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMDATABYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemDataByte1` writer - Second byte of AHB memory data"]
pub struct MEMDATABYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMDATABYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Second byte of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte1(&self) -> MEMDATABYTE1_R {
        MEMDATABYTE1_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Second byte of AHB memory data"]
    #[inline(always)]
    pub fn mem_data_byte1(&mut self) -> MEMDATABYTE1_W {
        MEMDATABYTE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second byte of AHB memory data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_data_byte1](index.html) module"]
pub struct MEMDATABYTE1_SPEC;
impl crate::RegisterSpec for MEMDATABYTE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_data_byte1::R](R) reader structure"]
impl crate::Readable for MEMDATABYTE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_data_byte1::W](W) writer structure"]
impl crate::Writable for MEMDATABYTE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemDataByte1 to value 0"]
impl crate::Resettable for MEMDATABYTE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
