#[doc = "Register `GPIO_INTR_RAW` reader"]
pub struct R(crate::R<GPIO_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_0_INTR_RAW` reader - Raw interrupt for GPIO_0. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_0_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_0_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_0_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_0_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_1_INTR_RAW` reader - Raw interrupt for GPIO_1. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_1_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_1_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_1_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_1_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_2_INTR_RAW` reader - Raw interrupt for GPIO_2. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_2_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_2_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_2_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_2_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_3_INTR_RAW` reader - Raw interrupt for GPIO_3. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_3_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_3_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_3_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_3_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_4_INTR_RAW` reader - Raw interrupt for GPIO_4. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_4_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_4_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_4_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_4_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_5_INTR_RAW` reader - Raw interrupt for GPIO_5. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_5_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_5_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_5_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_5_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_6_INTR_RAW` reader - Raw interrupt for GPIO_6. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_6_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_6_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_6_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_6_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_7_INTR_RAW` reader - Raw interrupt for GPIO_7. This register will reflect the value of the IO regardless of the type/polarity"]
pub struct GPIO_7_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl GPIO_7_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_7_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_7_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt for GPIO_0. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_0_intr_raw(&self) -> GPIO_0_INTR_RAW_R {
        GPIO_0_INTR_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt for GPIO_1. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_1_intr_raw(&self) -> GPIO_1_INTR_RAW_R {
        GPIO_1_INTR_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt for GPIO_2. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_2_intr_raw(&self) -> GPIO_2_INTR_RAW_R {
        GPIO_2_INTR_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt for GPIO_3. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_3_intr_raw(&self) -> GPIO_3_INTR_RAW_R {
        GPIO_3_INTR_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Raw interrupt for GPIO_4. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_4_intr_raw(&self) -> GPIO_4_INTR_RAW_R {
        GPIO_4_INTR_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt for GPIO_5. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_5_intr_raw(&self) -> GPIO_5_INTR_RAW_R {
        GPIO_5_INTR_RAW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt for GPIO_6. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_6_intr_raw(&self) -> GPIO_6_INTR_RAW_R {
        GPIO_6_INTR_RAW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Raw interrupt for GPIO_7. This register will reflect the value of the IO regardless of the type/polarity"]
    #[inline(always)]
    pub fn gpio_7_intr_raw(&self) -> GPIO_7_INTR_RAW_R {
        GPIO_7_INTR_RAW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "GPIO raw interrupt indicators\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_intr_raw](index.html) module"]
pub struct GPIO_INTR_RAW_SPEC;
impl crate::RegisterSpec for GPIO_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_intr_raw::R](R) reader structure"]
impl crate::Readable for GPIO_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_INTR_RAW to value 0"]
impl crate::Resettable for GPIO_INTR_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
