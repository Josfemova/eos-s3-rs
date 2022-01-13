#[doc = "Register `MemAddrByte1` reader"]
pub struct R(crate::R<MEMADDRBYTE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMADDRBYTE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMADDRBYTE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMADDRBYTE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MemAddrByte1` writer"]
pub struct W(crate::W<MEMADDRBYTE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMADDRBYTE1_SPEC>;
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
impl From<crate::W<MEMADDRBYTE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMADDRBYTE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MemAddrByte1` reader - AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
pub struct MEMADDRBYTE1_R(crate::FieldReader<u8, u8>);
impl MEMADDRBYTE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEMADDRBYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMADDRBYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MemAddrByte1` writer - AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
pub struct MEMADDRBYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMADDRBYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value as u8;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
    #[inline(always)]
    pub fn mem_addr_byte1(&self) -> MEMADDRBYTE1_R {
        MEMADDRBYTE1_R::new(self.bits as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail."]
    #[inline(always)]
    pub fn mem_addr_byte1(&mut self) -> MEMADDRBYTE1_W {
        MEMADDRBYTE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Memory Address, It is representing AHB Byte Address bit \\[15:8\\]. Bit 7:0 R/W, Default All 0 \n Once write to this SFR, an AHB memory Read Could be Trigger. See 'TLC AHB Memory Read Trigger' worksheet for detail.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_addr_byte1](index.html) module"]
pub struct MEMADDRBYTE1_SPEC;
impl crate::RegisterSpec for MEMADDRBYTE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mem_addr_byte1::R](R) reader structure"]
impl crate::Readable for MEMADDRBYTE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_addr_byte1::W](W) writer structure"]
impl crate::Writable for MEMADDRBYTE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MemAddrByte1 to value 0"]
impl crate::Resettable for MEMADDRBYTE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
