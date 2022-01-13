#[doc = "Register `MemDataByte3` reader"]
pub struct R(crate::R<MEMDATABYTE3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMDATABYTE3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMDATABYTE3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMDATABYTE3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemDataByte3` writer"]
pub struct W(crate::W<MEMDATABYTE3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMDATABYTE3_SPEC>;
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
impl From<crate::W<MEMDATABYTE3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMDATABYTE3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemDataByte3` reader - Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
pub struct MEMDATABYTE3_R(crate::FieldReader<u8, u8>);
impl MEMDATABYTE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMDATABYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMDATABYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemDataByte3` writer - Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
pub struct MEMDATABYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMDATABYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
    #[inline(always)]
    pub fn mem_data_byte3(&self) -> MEMDATABYTE3_R {
        MEMDATABYTE3_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)"]
    #[inline(always)]
    pub fn mem_data_byte3(&mut self) -> MEMDATABYTE3_W {
        MEMDATABYTE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Forth byte of AHB memory data, once write to this SFR \n 1. Trigger a AHB memory Write \n 2. Auto Increment the AHB memory address (MemAddrByte0/MemAddrByte1) by 4 since AHB memory Address is in Byte Granunality. (offset 0x20~0x21, 64KB range)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_data_byte3](index.html) module"]
pub struct MEMDATABYTE3_SPEC;
impl crate::RegisterSpec for MEMDATABYTE3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_data_byte3::R](R) reader structure"]
impl crate::Readable for MEMDATABYTE3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_data_byte3::W](W) writer structure"]
impl crate::Writable for MEMDATABYTE3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemDataByte3 to value 0"]
impl crate::Resettable for MEMDATABYTE3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
