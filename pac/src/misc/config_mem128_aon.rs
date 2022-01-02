#[doc = "Register `CONFIG_MEM128_AON` reader"]
pub struct R(crate::R<CONFIG_MEM128_AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_MEM128_AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_MEM128_AON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_MEM128_AON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_MEM128_AON` writer"]
pub struct W(crate::W<CONFIG_MEM128_AON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_MEM128_AON_SPEC>;
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
impl From<crate::W<CONFIG_MEM128_AON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_MEM128_AON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM0_32K_RM` reader - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM0_32K_RM_R(crate::FieldReader<u8, u8>);
impl MEM0_32K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM0_32K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM0_32K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM0_32K_RM` writer - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM0_32K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM0_32K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MEM0_32K_DST` reader - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM0_32K_DST_R(crate::FieldReader<bool, bool>);
impl MEM0_32K_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM0_32K_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM0_32K_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM0_32K_DST` writer - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM0_32K_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM0_32K_DST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MEM1_32K_RM` reader - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM1_32K_RM_R(crate::FieldReader<u8, u8>);
impl MEM1_32K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM1_32K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_32K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_32K_RM` writer - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM1_32K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_32K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `MEM1_32K_DST` reader - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM1_32K_DST_R(crate::FieldReader<bool, bool>);
impl MEM1_32K_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM1_32K_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM1_32K_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM1_32K_DST` writer - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM1_32K_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM1_32K_DST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `MEM2_32K_RM` reader - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM2_32K_RM_R(crate::FieldReader<u8, u8>);
impl MEM2_32K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM2_32K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_32K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_32K_RM` writer - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM2_32K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_32K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `MEM2_32K_DST` reader - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM2_32K_DST_R(crate::FieldReader<bool, bool>);
impl MEM2_32K_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM2_32K_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM2_32K_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM2_32K_DST` writer - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM2_32K_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM2_32K_DST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `MEM3_32K_RM` reader - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM3_32K_RM_R(crate::FieldReader<u8, u8>);
impl MEM3_32K_RM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MEM3_32K_RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_32K_RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM3_32K_RM` writer - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
pub struct MEM3_32K_RM_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM3_32K_RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `MEM3_32K_DST` reader - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
pub struct MEM3_32K_DST_R(crate::FieldReader<bool, bool>);
impl MEM3_32K_DST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM3_32K_DST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM3_32K_DST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem0_32k_rm(&self) -> MEM0_32K_RM_R {
        MEM0_32K_RM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem0_32k_dst(&self) -> MEM0_32K_DST_R {
        MEM0_32K_DST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem1_32k_rm(&self) -> MEM1_32K_RM_R {
        MEM1_32K_RM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem1_32k_dst(&self) -> MEM1_32K_DST_R {
        MEM1_32K_DST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem2_32k_rm(&self) -> MEM2_32K_RM_R {
        MEM2_32K_RM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem2_32k_dst(&self) -> MEM2_32K_DST_R {
        MEM2_32K_DST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem3_32k_rm(&self) -> MEM3_32K_RM_R {
        MEM3_32K_RM_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem3_32k_dst(&self) -> MEM3_32K_DST_R {
        MEM3_32K_DST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem0_32k_rm(&mut self) -> MEM0_32K_RM_W {
        MEM0_32K_RM_W { w: self }
    }
    #[doc = "Bit 4 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem0_32k_dst(&mut self) -> MEM0_32K_DST_W {
        MEM0_32K_DST_W { w: self }
    }
    #[doc = "Bits 5:6 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem1_32k_rm(&mut self) -> MEM1_32K_RM_W {
        MEM1_32K_RM_W { w: self }
    }
    #[doc = "Bit 9 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem1_32k_dst(&mut self) -> MEM1_32K_DST_W {
        MEM1_32K_DST_W { w: self }
    }
    #[doc = "Bits 10:11 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem2_32k_rm(&mut self) -> MEM2_32K_RM_W {
        MEM2_32K_RM_W { w: self }
    }
    #[doc = "Bit 14 - Disable-Self-Time. When asserted high, overrides the self-timed circuitry and causes the read margin to be controlled by the falling clk edge. Requires margin\\[\\]
to be set to 2'b00. This pin is intended for debug/FA purposes only."]
    #[inline(always)]
    pub fn mem2_32k_dst(&mut self) -> MEM2_32K_DST_W {
        MEM2_32K_DST_W { w: self }
    }
    #[doc = "Bits 15:16 - Read and write margin control. Recommended setting is 2'b10. 2'b00 provides the most margin (slowest speed). 2'b11 provides the least margin (fastest speed) memory. This setting is required for VDDMIN operation."]
    #[inline(always)]
    pub fn mem3_32k_rm(&mut self) -> MEM3_32K_RM_W {
        MEM3_32K_RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_mem128_aon](index.html) module"]
pub struct CONFIG_MEM128_AON_SPEC;
impl crate::RegisterSpec for CONFIG_MEM128_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_mem128_aon::R](R) reader structure"]
impl crate::Readable for CONFIG_MEM128_AON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_mem128_aon::W](W) writer structure"]
impl crate::Writable for CONFIG_MEM128_AON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG_MEM128_AON to value 0x0001_0842"]
impl crate::Resettable for CONFIG_MEM128_AON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0842
    }
}
