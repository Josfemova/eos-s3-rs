#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Addr` reader - Slave address register via Wishbone master"]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Addr` writer - Slave address register via Wishbone master"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "MSB to select which which slave accessed by WB master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLAVE_SEL_A {
    #[doc = "0: Select i2c_0 as the sensor that will be accessed by the Wishbone Master"]
    I2C_0 = 0,
    #[doc = "1: Select i2c_1 as the sensor that will be accessed by the Wishbone Master"]
    I2C_1 = 1,
    #[doc = "2: Select spi_0 as the sensor that will be accessed by the Wishbone Master"]
    SPI_0 = 2,
}
impl From<SLAVE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SLAVE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `slave_sel` reader - MSB to select which which slave accessed by WB master"]
pub struct SLAVE_SEL_R(crate::FieldReader<u8, SLAVE_SEL_A>);
impl SLAVE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLAVE_SEL_A> {
        match self.bits {
            0 => Some(SLAVE_SEL_A::I2C_0),
            1 => Some(SLAVE_SEL_A::I2C_1),
            2 => Some(SLAVE_SEL_A::SPI_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C_0`"]
    #[inline(always)]
    pub fn is_i2c_0(&self) -> bool {
        **self == SLAVE_SEL_A::I2C_0
    }
    #[doc = "Checks if the value of the field is `I2C_1`"]
    #[inline(always)]
    pub fn is_i2c_1(&self) -> bool {
        **self == SLAVE_SEL_A::I2C_1
    }
    #[doc = "Checks if the value of the field is `SPI_0`"]
    #[inline(always)]
    pub fn is_spi_0(&self) -> bool {
        **self == SLAVE_SEL_A::SPI_0
    }
}
impl core::ops::Deref for SLAVE_SEL_R {
    type Target = crate::FieldReader<u8, SLAVE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_sel` writer - MSB to select which which slave accessed by WB master"]
pub struct SLAVE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select i2c_0 as the sensor that will be accessed by the Wishbone Master"]
    #[inline(always)]
    pub fn i2c_0(self) -> &'a mut W {
        self.variant(SLAVE_SEL_A::I2C_0)
    }
    #[doc = "Select i2c_1 as the sensor that will be accessed by the Wishbone Master"]
    #[inline(always)]
    pub fn i2c_1(self) -> &'a mut W {
        self.variant(SLAVE_SEL_A::I2C_1)
    }
    #[doc = "Select spi_0 as the sensor that will be accessed by the Wishbone Master"]
    #[inline(always)]
    pub fn spi_0(self) -> &'a mut W {
        self.variant(SLAVE_SEL_A::SPI_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave address register via Wishbone master"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - MSB to select which which slave accessed by WB master"]
    #[inline(always)]
    pub fn slave_sel(&self) -> SLAVE_SEL_R {
        SLAVE_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave address register via Wishbone master"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 6:7 - MSB to select which which slave accessed by WB master"]
    #[inline(always)]
    pub fn slave_sel(&mut self) -> SLAVE_SEL_W {
        SLAVE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wishbone master address selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
