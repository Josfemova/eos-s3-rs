#[doc = "Register `FFE_INTR` reader"]
pub struct R(crate::R<FFE_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFE_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFE_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFE_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFE_INTR` writer"]
pub struct W(crate::W<FFE_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFE_INTR_SPEC>;
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
impl From<crate::W<FFE_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFE_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFE0_0_INTR` reader - Active high edge interrupt detected for FFE0_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_0_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_0_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_0_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_0_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_0_INTR` writer - Active high edge interrupt detected for FFE0_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_0_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_0_INTR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FFE0_1_INTR` reader - Active high edge interrupt detected for FFE0_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_1_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_1_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_1_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_1_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_1_INTR` writer - Active high edge interrupt detected for FFE0_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_1_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_1_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `FFE0_2_INTR` reader - Active high edge interrupt detected for FFE0_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_2_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_2_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_2_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_2_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_2_INTR` writer - Active high edge interrupt detected for FFE0_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_2_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_2_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FFE0_3_INTR` reader - Active high edge interrupt detected for FFE0_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_3_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_3_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_3_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_3_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_3_INTR` writer - Active high edge interrupt detected for FFE0_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_3_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_3_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `FFE0_4_INTR` reader - Active high edge interrupt detected for FFE0_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_4_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_4_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_4_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_4_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_4_INTR` writer - Active high edge interrupt detected for FFE0_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_4_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_4_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FFE0_5_INTR` reader - Active high edge interrupt detected for FFE0_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_5_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_5_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_5_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_5_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_5_INTR` writer - Active high edge interrupt detected for FFE0_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_5_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_5_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `FFE0_6_INTR` reader - Active high edge interrupt detected for FFE0_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_6_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_6_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_6_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_6_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_6_INTR` writer - Active high edge interrupt detected for FFE0_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_6_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_6_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FFE0_7_INTR` reader - Active high edge interrupt detected for FFE0_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_7_INTR_R(crate::FieldReader<bool, bool>);
impl FFE0_7_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFE0_7_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFE0_7_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFE0_7_INTR` writer - Active high edge interrupt detected for FFE0_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct FFE0_7_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FFE0_7_INTR_W<'a> {
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
            (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Active high edge interrupt detected for FFE0_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_0_intr(&self) -> FFE0_0_INTR_R {
        FFE0_0_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for FFE0_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_1_intr(&self) -> FFE0_1_INTR_R {
        FFE0_1_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for FFE0_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_2_intr(&self) -> FFE0_2_INTR_R {
        FFE0_2_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for FFE0_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_3_intr(&self) -> FFE0_3_INTR_R {
        FFE0_3_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active high edge interrupt detected for FFE0_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_4_intr(&self) -> FFE0_4_INTR_R {
        FFE0_4_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Active high edge interrupt detected for FFE0_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_5_intr(&self) -> FFE0_5_INTR_R {
        FFE0_5_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Active high edge interrupt detected for FFE0_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_6_intr(&self) -> FFE0_6_INTR_R {
        FFE0_6_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Active high edge interrupt detected for FFE0_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_7_intr(&self) -> FFE0_7_INTR_R {
        FFE0_7_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high edge interrupt detected for FFE0_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_0_intr(&mut self) -> FFE0_0_INTR_W {
        FFE0_0_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for FFE0_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_1_intr(&mut self) -> FFE0_1_INTR_W {
        FFE0_1_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for FFE0_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_2_intr(&mut self) -> FFE0_2_INTR_W {
        FFE0_2_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for FFE0_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_3_intr(&mut self) -> FFE0_3_INTR_W {
        FFE0_3_INTR_W { w: self }
    }
    #[doc = "Bit 4 - Active high edge interrupt detected for FFE0_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_4_intr(&mut self) -> FFE0_4_INTR_W {
        FFE0_4_INTR_W { w: self }
    }
    #[doc = "Bit 5 - Active high edge interrupt detected for FFE0_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_5_intr(&mut self) -> FFE0_5_INTR_W {
        FFE0_5_INTR_W { w: self }
    }
    #[doc = "Bit 6 - Active high edge interrupt detected for FFE0_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_6_intr(&mut self) -> FFE0_6_INTR_W {
        FFE0_6_INTR_W { w: self }
    }
    #[doc = "Bit 7 - Active high edge interrupt detected for FFE0_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn ffe0_7_intr(&mut self) -> FFE0_7_INTR_W {
        FFE0_7_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of FFE0 interrupt triggers detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffe_intr](index.html) module"]
pub struct FFE_INTR_SPEC;
impl crate::RegisterSpec for FFE_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffe_intr::R](R) reader structure"]
impl crate::Readable for FFE_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffe_intr::W](W) writer structure"]
impl crate::Writable for FFE_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFE_INTR to value 0"]
impl crate::Resettable for FFE_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
