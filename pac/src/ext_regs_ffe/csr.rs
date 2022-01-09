#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wb_ms_start` reader - Wishbone master start transactions. This bit is self clearing."]
pub struct WB_MS_START_R(crate::FieldReader<bool, bool>);
impl WB_MS_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WB_MS_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WB_MS_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wb_ms_start` writer - Wishbone master start transactions. This bit is self clearing."]
pub struct WB_MS_START_W<'a> {
    w: &'a mut W,
}
impl<'a> WB_MS_START_W<'a> {
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
#[doc = "Wishbone master write enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WB_MS_WEN_A {
    #[doc = "0: read"]
    READ = 0,
    #[doc = "1: write"]
    WRITE = 1,
}
impl From<WB_MS_WEN_A> for bool {
    #[inline(always)]
    fn from(variant: WB_MS_WEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `wb_ms_wen` reader - Wishbone master write enable."]
pub struct WB_MS_WEN_R(crate::FieldReader<bool, WB_MS_WEN_A>);
impl WB_MS_WEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WB_MS_WEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WB_MS_WEN_A {
        match self.bits {
            false => WB_MS_WEN_A::READ,
            true => WB_MS_WEN_A::WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == WB_MS_WEN_A::READ
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == WB_MS_WEN_A::WRITE
    }
}
impl core::ops::Deref for WB_MS_WEN_R {
    type Target = crate::FieldReader<bool, WB_MS_WEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wb_ms_wen` writer - Wishbone master write enable."]
pub struct WB_MS_WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WB_MS_WEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WB_MS_WEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(WB_MS_WEN_A::READ)
    }
    #[doc = "write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(WB_MS_WEN_A::WRITE)
    }
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
#[doc = "Mux select between SM and WB masters.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_WB_SM_AW {
    #[doc = "0: Select SM WB Master."]
    SM_WISHBONE_MASTER = 0,
    #[doc = "1: Select WB Master."]
    WISHBONE_MASTER = 1,
}
impl From<MUX_WB_SM_AW> for bool {
    #[inline(always)]
    fn from(variant: MUX_WB_SM_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `mux_wb_sm` writer - Mux select between SM and WB masters."]
pub struct MUX_WB_SM_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_WB_SM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_WB_SM_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select SM WB Master."]
    #[inline(always)]
    pub fn sm_wishbone_master(self) -> &'a mut W {
        self.variant(MUX_WB_SM_AW::SM_WISHBONE_MASTER)
    }
    #[doc = "Select WB Master."]
    #[inline(always)]
    pub fn wishbone_master(self) -> &'a mut W {
        self.variant(MUX_WB_SM_AW::WISHBONE_MASTER)
    }
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
#[doc = "Field `BUSY` reader - Indicates if the Wishbone is busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFL` reader - Indicates if there's a FFE push operation overflow"]
pub struct OVFL_R(crate::FieldReader<bool, bool>);
impl OVFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "I2C_0 wishbone control mux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_MUX_SEL_A {
    #[doc = "0: Let SM1 control the port."]
    SM1_AS_CONTROLLER = 0,
    #[doc = "1: Let Wishbone Master control the port"]
    WB_MASTER_AS_CONTROLLER = 1,
}
impl From<I2C0_MUX_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_MUX_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `i2c0_mux_sel` reader - I2C_0 wishbone control mux select"]
pub struct I2C0_MUX_SEL_R(crate::FieldReader<bool, I2C0_MUX_SEL_A>);
impl I2C0_MUX_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_MUX_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_MUX_SEL_A {
        match self.bits {
            false => I2C0_MUX_SEL_A::SM1_AS_CONTROLLER,
            true => I2C0_MUX_SEL_A::WB_MASTER_AS_CONTROLLER,
        }
    }
    #[doc = "Checks if the value of the field is `SM1_AS_CONTROLLER`"]
    #[inline(always)]
    pub fn is_sm1_as_controller(&self) -> bool {
        **self == I2C0_MUX_SEL_A::SM1_AS_CONTROLLER
    }
    #[doc = "Checks if the value of the field is `WB_MASTER_AS_CONTROLLER`"]
    #[inline(always)]
    pub fn is_wb_master_as_controller(&self) -> bool {
        **self == I2C0_MUX_SEL_A::WB_MASTER_AS_CONTROLLER
    }
}
impl core::ops::Deref for I2C0_MUX_SEL_R {
    type Target = crate::FieldReader<bool, I2C0_MUX_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c0_mux_sel` writer - I2C_0 wishbone control mux select"]
pub struct I2C0_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_MUX_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_MUX_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Let SM1 control the port."]
    #[inline(always)]
    pub fn sm1_as_controller(self) -> &'a mut W {
        self.variant(I2C0_MUX_SEL_A::SM1_AS_CONTROLLER)
    }
    #[doc = "Let Wishbone Master control the port"]
    #[inline(always)]
    pub fn wb_master_as_controller(self) -> &'a mut W {
        self.variant(I2C0_MUX_SEL_A::WB_MASTER_AS_CONTROLLER)
    }
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
#[doc = "I2C_1 wishbone control mux select"]
pub type I2C1_MUX_SEL_A = I2C0_MUX_SEL_A;
#[doc = "Field `i2c1_mux_sel` reader - I2C_1 wishbone control mux select"]
pub type I2C1_MUX_SEL_R = I2C0_MUX_SEL_R;
#[doc = "Field `i2c1_mux_sel` writer - I2C_1 wishbone control mux select"]
pub struct I2C1_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_MUX_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_MUX_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Let SM1 control the port."]
    #[inline(always)]
    pub fn sm1_as_controller(self) -> &'a mut W {
        self.variant(I2C1_MUX_SEL_A::SM1_AS_CONTROLLER)
    }
    #[doc = "Let Wishbone Master control the port"]
    #[inline(always)]
    pub fn wb_master_as_controller(self) -> &'a mut W {
        self.variant(I2C1_MUX_SEL_A::WB_MASTER_AS_CONTROLLER)
    }
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
#[doc = "spi_0 wishbone control mux select"]
pub type SPI0_MUX_SEL_A = I2C0_MUX_SEL_A;
#[doc = "Field `spi0_mux_sel` reader - spi_0 wishbone control mux select"]
pub type SPI0_MUX_SEL_R = I2C0_MUX_SEL_R;
#[doc = "Field `spi0_mux_sel` writer - spi_0 wishbone control mux select"]
pub struct SPI0_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MUX_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_MUX_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Let SM1 control the port."]
    #[inline(always)]
    pub fn sm1_as_controller(self) -> &'a mut W {
        self.variant(SPI0_MUX_SEL_A::SM1_AS_CONTROLLER)
    }
    #[doc = "Let Wishbone Master control the port"]
    #[inline(always)]
    pub fn wb_master_as_controller(self) -> &'a mut W {
        self.variant(SPI0_MUX_SEL_A::WB_MASTER_AS_CONTROLLER)
    }
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
    #[doc = "Bit 0 - Wishbone master start transactions. This bit is self clearing."]
    #[inline(always)]
    pub fn wb_ms_start(&self) -> WB_MS_START_R {
        WB_MS_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wishbone master write enable."]
    #[inline(always)]
    pub fn wb_ms_wen(&self) -> WB_MS_WEN_R {
        WB_MS_WEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates if the Wishbone is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates if there's a FFE push operation overflow"]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - I2C_0 wishbone control mux select"]
    #[inline(always)]
    pub fn i2c0_mux_sel(&self) -> I2C0_MUX_SEL_R {
        I2C0_MUX_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C_1 wishbone control mux select"]
    #[inline(always)]
    pub fn i2c1_mux_sel(&self) -> I2C1_MUX_SEL_R {
        I2C1_MUX_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - spi_0 wishbone control mux select"]
    #[inline(always)]
    pub fn spi0_mux_sel(&self) -> SPI0_MUX_SEL_R {
        SPI0_MUX_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wishbone master start transactions. This bit is self clearing."]
    #[inline(always)]
    pub fn wb_ms_start(&mut self) -> WB_MS_START_W {
        WB_MS_START_W { w: self }
    }
    #[doc = "Bit 1 - Wishbone master write enable."]
    #[inline(always)]
    pub fn wb_ms_wen(&mut self) -> WB_MS_WEN_W {
        WB_MS_WEN_W { w: self }
    }
    #[doc = "Bit 2 - Mux select between SM and WB masters."]
    #[inline(always)]
    pub fn mux_wb_sm(&mut self) -> MUX_WB_SM_W {
        MUX_WB_SM_W { w: self }
    }
    #[doc = "Bit 5 - I2C_0 wishbone control mux select"]
    #[inline(always)]
    pub fn i2c0_mux_sel(&mut self) -> I2C0_MUX_SEL_W {
        I2C0_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 6 - I2C_1 wishbone control mux select"]
    #[inline(always)]
    pub fn i2c1_mux_sel(&mut self) -> I2C1_MUX_SEL_W {
        I2C1_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 7 - spi_0 wishbone control mux select"]
    #[inline(always)]
    pub fn spi0_mux_sel(&mut self) -> SPI0_MUX_SEL_W {
        SPI0_MUX_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
