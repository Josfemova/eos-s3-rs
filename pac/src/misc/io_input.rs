#[doc = "Register `IO_INPUT` reader"]
pub struct R(crate::R<IO_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IO_0` reader - Read digital value of pin mapped to IO bit 0"]
pub struct IO_0_R(crate::FieldReader<bool, bool>);
impl IO_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_1` reader - Read digital value of pin mapped to IO bit 1"]
pub struct IO_1_R(crate::FieldReader<bool, bool>);
impl IO_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_2` reader - Read digital value of pin mapped to IO bit 2"]
pub struct IO_2_R(crate::FieldReader<bool, bool>);
impl IO_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_3` reader - Read digital value of pin mapped to IO bit 3"]
pub struct IO_3_R(crate::FieldReader<bool, bool>);
impl IO_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_4` reader - Read digital value of pin mapped to IO bit 4"]
pub struct IO_4_R(crate::FieldReader<bool, bool>);
impl IO_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_5` reader - Read digital value of pin mapped to IO bit 5"]
pub struct IO_5_R(crate::FieldReader<bool, bool>);
impl IO_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_6` reader - Read digital value of pin mapped to IO bit 6"]
pub struct IO_6_R(crate::FieldReader<bool, bool>);
impl IO_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_7` reader - Read digital value of pin mapped to IO bit 7"]
pub struct IO_7_R(crate::FieldReader<bool, bool>);
impl IO_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read digital value of pin mapped to IO bit 0"]
    #[inline(always)]
    pub fn io_0(&self) -> IO_0_R {
        IO_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read digital value of pin mapped to IO bit 1"]
    #[inline(always)]
    pub fn io_1(&self) -> IO_1_R {
        IO_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read digital value of pin mapped to IO bit 2"]
    #[inline(always)]
    pub fn io_2(&self) -> IO_2_R {
        IO_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read digital value of pin mapped to IO bit 3"]
    #[inline(always)]
    pub fn io_3(&self) -> IO_3_R {
        IO_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read digital value of pin mapped to IO bit 4"]
    #[inline(always)]
    pub fn io_4(&self) -> IO_4_R {
        IO_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read digital value of pin mapped to IO bit 5"]
    #[inline(always)]
    pub fn io_5(&self) -> IO_5_R {
        IO_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read digital value of pin mapped to IO bit 6"]
    #[inline(always)]
    pub fn io_6(&self) -> IO_6_R {
        IO_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read digital value of pin mapped to IO bit 7"]
    #[inline(always)]
    pub fn io_7(&self) -> IO_7_R {
        IO_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Reads the value of the IO pins\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_input](index.html) module"]
pub struct IO_INPUT_SPEC;
impl crate::RegisterSpec for IO_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_input::R](R) reader structure"]
impl crate::Readable for IO_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IO_INPUT to value 0"]
impl crate::Resettable for IO_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
