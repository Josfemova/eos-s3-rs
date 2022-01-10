#[doc = "Register `GPIO_INTR` reader"]
pub struct R(crate::R<GPIO_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INTR` writer"]
pub struct W(crate::W<GPIO_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INTR_SPEC>;
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
impl From<crate::W<GPIO_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_0_INTR` reader - Active high edge interrupt detected for GPIO_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_0_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_0_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_0_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_0_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_0_INTR` writer - Active high edge interrupt detected for GPIO_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_0_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_0_INTR_W<'a> {
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
#[doc = "Field `GPIO_1_INTR` reader - Active high edge interrupt detected for GPIO_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_1_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_1_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_1_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_1_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_1_INTR` writer - Active high edge interrupt detected for GPIO_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_1_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_1_INTR_W<'a> {
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
#[doc = "Field `GPIO_2_INTR` reader - Active high edge interrupt detected for GPIO_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_2_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_2_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_2_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_2_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_2_INTR` writer - Active high edge interrupt detected for GPIO_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_2_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_2_INTR_W<'a> {
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
#[doc = "Field `GPIO_3_INTR` reader - Active high edge interrupt detected for GPIO_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_3_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_3_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_3_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_3_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_3_INTR` writer - Active high edge interrupt detected for GPIO_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_3_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_3_INTR_W<'a> {
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
#[doc = "Field `GPIO_4_INTR` reader - Active high edge interrupt detected for GPIO_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_4_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_4_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_4_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_4_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_4_INTR` writer - Active high edge interrupt detected for GPIO_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_4_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_4_INTR_W<'a> {
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
#[doc = "Field `GPIO_5_INTR` reader - Active high edge interrupt detected for GPIO_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_5_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_5_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_5_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_5_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_5_INTR` writer - Active high edge interrupt detected for GPIO_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_5_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_5_INTR_W<'a> {
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
#[doc = "Field `GPIO_6_INTR` reader - Active high edge interrupt detected for GPIO_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_6_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_6_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_6_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_6_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_6_INTR` writer - Active high edge interrupt detected for GPIO_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_6_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_6_INTR_W<'a> {
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
#[doc = "Field `GPIO_7_INTR` reader - Active high edge interrupt detected for GPIO_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_7_INTR_R(crate::FieldReader<bool, bool>);
impl GPIO_7_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_7_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_7_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_7_INTR` writer - Active high edge interrupt detected for GPIO_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
pub struct GPIO_7_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_7_INTR_W<'a> {
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
    #[doc = "Bit 0 - Active high edge interrupt detected for GPIO_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_0_intr(&self) -> GPIO_0_INTR_R {
        GPIO_0_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for GPIO_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_1_intr(&self) -> GPIO_1_INTR_R {
        GPIO_1_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for GPIO_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_2_intr(&self) -> GPIO_2_INTR_R {
        GPIO_2_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for GPIO_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_3_intr(&self) -> GPIO_3_INTR_R {
        GPIO_3_INTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active high edge interrupt detected for GPIO_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_4_intr(&self) -> GPIO_4_INTR_R {
        GPIO_4_INTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Active high edge interrupt detected for GPIO_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_5_intr(&self) -> GPIO_5_INTR_R {
        GPIO_5_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Active high edge interrupt detected for GPIO_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_6_intr(&self) -> GPIO_6_INTR_R {
        GPIO_6_INTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Active high edge interrupt detected for GPIO_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_7_intr(&self) -> GPIO_7_INTR_R {
        GPIO_7_INTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high edge interrupt detected for GPIO_0. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_0_intr(&mut self) -> GPIO_0_INTR_W {
        GPIO_0_INTR_W { w: self }
    }
    #[doc = "Bit 1 - Active high edge interrupt detected for GPIO_1. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_1_intr(&mut self) -> GPIO_1_INTR_W {
        GPIO_1_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Active high edge interrupt detected for GPIO_2. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_2_intr(&mut self) -> GPIO_2_INTR_W {
        GPIO_2_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Active high edge interrupt detected for GPIO_3. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_3_intr(&mut self) -> GPIO_3_INTR_W {
        GPIO_3_INTR_W { w: self }
    }
    #[doc = "Bit 4 - Active high edge interrupt detected for GPIO_4. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_4_intr(&mut self) -> GPIO_4_INTR_W {
        GPIO_4_INTR_W { w: self }
    }
    #[doc = "Bit 5 - Active high edge interrupt detected for GPIO_5. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_5_intr(&mut self) -> GPIO_5_INTR_W {
        GPIO_5_INTR_W { w: self }
    }
    #[doc = "Bit 6 - Active high edge interrupt detected for GPIO_6. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_6_intr(&mut self) -> GPIO_6_INTR_W {
        GPIO_6_INTR_W { w: self }
    }
    #[doc = "Bit 7 - Active high edge interrupt detected for GPIO_7. When interrupt type is selected as edge detect, this register will return high when triggered, write 1 to clear."]
    #[inline(always)]
    pub fn gpio_7_intr(&mut self) -> GPIO_7_INTR_W {
        GPIO_7_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicators of interrupt triggers detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_intr](index.html) module"]
pub struct GPIO_INTR_SPEC;
impl crate::RegisterSpec for GPIO_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_intr::R](R) reader structure"]
impl crate::Readable for GPIO_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_intr::W](W) writer structure"]
impl crate::Writable for GPIO_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INTR to value 0"]
impl crate::Resettable for GPIO_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
