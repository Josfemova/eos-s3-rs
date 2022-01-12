#[doc = "Register `SER` reader"]
pub struct R(crate::R<SER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SER` writer"]
pub struct W(crate::W<SER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SER_SPEC>;
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
impl From<crate::W<SER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SER_SS1` reader - Slave 1 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS1_R(crate::FieldReader<bool, bool>);
impl SER_SS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SER_SS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SER_SS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SER_SS1` writer - Slave 1 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_SS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `SER_SS2` reader - Slave 2 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS2_R(crate::FieldReader<bool, bool>);
impl SER_SS2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SER_SS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SER_SS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SER_SS2` writer - Slave 2 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_SS2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SER_SS3` reader - Slave 3 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS3_R(crate::FieldReader<bool, bool>);
impl SER_SS3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SER_SS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SER_SS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SER_SS3` writer - Slave 3 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
pub struct SER_SS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_SS3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x01 << 2)) | ((value as u8 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave 1 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss1(&self) -> SER_SS1_R {
        SER_SS1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave 2 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss2(&self) -> SER_SS2_R {
        SER_SS2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave 3 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss3(&self) -> SER_SS3_R {
        SER_SS3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave 1 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss1(&mut self) -> SER_SS1_W {
        SER_SS1_W { w: self }
    }
    #[doc = "Bit 1 - Slave 2 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss2(&mut self) -> SER_SS2_W {
        SER_SS2_W { w: self }
    }
    #[doc = "Bit 2 - Slave 3 Select Enable Flag. When set, the corresponding slave select line from the master is activated when a serial transfer begins. It should be noted that setting or clearing bits in this register have no effect on the corresponding slave select outputs until a transfer is started. Before beginning a transfer, you should enable the bit in this register. When not operating in broadcast mode, only one bit in the register should be set."]
    #[inline(always)]
    pub fn ser_ss3(&mut self) -> SER_SS3_W {
        SER_SS3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Enable Register: This register enables the individual slave select output lines from the SPI Master. You cannot write to this register when SPI Master is busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ser](index.html) module"]
pub struct SER_SPEC;
impl crate::RegisterSpec for SER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ser::R](R) reader structure"]
impl crate::Readable for SER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ser::W](W) writer structure"]
impl crate::Writable for SER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
