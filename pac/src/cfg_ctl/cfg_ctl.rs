#[doc = "Register `CFG_CTL` reader"]
pub struct R(crate::R<CFG_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_CTL` writer"]
pub struct W(crate::W<CFG_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_CTL_SPEC>;
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
impl From<crate::W<CFG_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SEL_CFG` reader - ARM firmware/software sets this register 1'b1 to Get the Control Right of Cfg_Ctl"]
pub struct APB_SEL_CFG_R(crate::FieldReader<bool, bool>);
impl APB_SEL_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_SEL_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_SEL_CFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_SEL_CFG` writer - ARM firmware/software sets this register 1'b1 to Get the Control Right of Cfg_Ctl"]
pub struct APB_SEL_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_SEL_CFG_W<'a> {
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
#[doc = "Field `APB_TRM_SEL` reader - ARM firmware/software sets this register 1'b1 to Select TRM Block"]
pub struct APB_TRM_SEL_R(crate::FieldReader<bool, bool>);
impl APB_TRM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_TRM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_TRM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_TRM_SEL` writer - ARM firmware/software sets this register 1'b1 to Select TRM Block"]
pub struct APB_TRM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_TRM_SEL_W<'a> {
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
#[doc = "Field `APB_TR_SEL` reader - ARM firmware/software sets this register 1'b1 to Select TR Block"]
pub struct APB_TR_SEL_R(crate::FieldReader<bool, bool>);
impl APB_TR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_TR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_TR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_TR_SEL` writer - ARM firmware/software sets this register 1'b1 to Select TR Block"]
pub struct APB_TR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_TR_SEL_W<'a> {
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
#[doc = "Field `APB_TLM_SEL` reader - ARM firmware/software sets this register 1'b1 to Select TLM Block"]
pub struct APB_TLM_SEL_R(crate::FieldReader<bool, bool>);
impl APB_TLM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_TLM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_TLM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_TLM_SEL` writer - ARM firmware/software sets this register 1'b1 to Select TLM Block"]
pub struct APB_TLM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_TLM_SEL_W<'a> {
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
#[doc = "Field `APB_TL_SEL` reader - ARM firmware/software sets this register 1'b1 to Select TL Block"]
pub struct APB_TL_SEL_R(crate::FieldReader<bool, bool>);
impl APB_TL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_TL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_TL_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_TL_SEL` writer - ARM firmware/software sets this register 1'b1 to Select TL Block"]
pub struct APB_TL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_TL_SEL_W<'a> {
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
#[doc = "Field `APB_BRM_SEL` reader - ARM firmware/software sets this register 1'b1 to Select BRM Block"]
pub struct APB_BRM_SEL_R(crate::FieldReader<bool, bool>);
impl APB_BRM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_BRM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_BRM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_BRM_SEL` writer - ARM firmware/software sets this register 1'b1 to Select BRM Block"]
pub struct APB_BRM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_BRM_SEL_W<'a> {
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
#[doc = "Field `APB_BR_SEL` reader - ARM firmware/software sets this register 1'b1 to Select BR Block"]
pub struct APB_BR_SEL_R(crate::FieldReader<bool, bool>);
impl APB_BR_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_BR_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_BR_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_BR_SEL` writer - ARM firmware/software sets this register 1'b1 to Select BR Block"]
pub struct APB_BR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_BR_SEL_W<'a> {
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
#[doc = "Field `APB_BLM_SEL` reader - ARM firmware/software sets this register 1'b1 to Select BLM Block"]
pub struct APB_BLM_SEL_R(crate::FieldReader<bool, bool>);
impl APB_BLM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_BLM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_BLM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_BLM_SEL` writer - ARM firmware/software sets this register 1'b1 to Select BLM Block"]
pub struct APB_BLM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_BLM_SEL_W<'a> {
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
#[doc = "Field `APB_BL_SEL` reader - ARM firmware/software sets this register 1'b1 to Select BL Block"]
pub struct APB_BL_SEL_R(crate::FieldReader<bool, bool>);
impl APB_BL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_BL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_BL_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_BL_SEL` writer - ARM firmware/software sets this register 1'b1 to Select BL Block"]
pub struct APB_BL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_BL_SEL_W<'a> {
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `APB_PARTIAL_LOAD` reader - ARM firmware/software sets this register 1'b1 to enable Partial Load"]
pub struct APB_PARTIAL_LOAD_R(crate::FieldReader<bool, bool>);
impl APB_PARTIAL_LOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_PARTIAL_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_PARTIAL_LOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_PARTIAL_LOAD` writer - ARM firmware/software sets this register 1'b1 to enable Partial Load"]
pub struct APB_PARTIAL_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_PARTIAL_LOAD_W<'a> {
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `APB_WL_DIN` reader - ARM firmware/software sets this register to set Word Line Data In"]
pub struct APB_WL_DIN_R(crate::FieldReader<u8, u8>);
impl APB_WL_DIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APB_WL_DIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_WL_DIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_WL_DIN` writer - ARM firmware/software sets this register to set Word Line Data In"]
pub struct APB_WL_DIN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_WL_DIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `APB_CFG_RD` reader - ARM firmware/software sets this register 1'b1 and APB_CFG_WR 1'b0 to perform Configuration Read"]
pub struct APB_CFG_RD_R(crate::FieldReader<bool, bool>);
impl APB_CFG_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_CFG_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_CFG_RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_CFG_RD` writer - ARM firmware/software sets this register 1'b1 and APB_CFG_WR 1'b0 to perform Configuration Read"]
pub struct APB_CFG_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CFG_RD_W<'a> {
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
            (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `APB_CFG_WR` reader - ARM firmware/software sets this register 1'b1 and APB_CFG_RD 1'b0 to perform Configuration Write"]
pub struct APB_CFG_WR_R(crate::FieldReader<bool, bool>);
impl APB_CFG_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_CFG_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_CFG_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_CFG_WR` writer - ARM firmware/software sets this register 1'b1 and APB_CFG_RD 1'b0 to perform Configuration Write"]
pub struct APB_CFG_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CFG_WR_W<'a> {
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
            (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SW_PWR_GATE` reader - Shift Register Power Gate Status: 1 in configuration mode, 0 in normal mode"]
pub struct SW_PWR_GATE_R(crate::FieldReader<bool, bool>);
impl SW_PWR_GATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_PWR_GATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_PWR_GATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_PWR_GATE` writer - Shift Register Power Gate Status: 1 in configuration mode, 0 in normal mode"]
pub struct SW_PWR_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_PWR_GATE_W<'a> {
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
            (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ARM firmware/software sets this register 1'b1 to Get the Control Right of Cfg_Ctl"]
    #[inline(always)]
    pub fn apb_sel_cfg(&self) -> APB_SEL_CFG_R {
        APB_SEL_CFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ARM firmware/software sets this register 1'b1 to Select TRM Block"]
    #[inline(always)]
    pub fn apb_trm_sel(&self) -> APB_TRM_SEL_R {
        APB_TRM_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ARM firmware/software sets this register 1'b1 to Select TR Block"]
    #[inline(always)]
    pub fn apb_tr_sel(&self) -> APB_TR_SEL_R {
        APB_TR_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ARM firmware/software sets this register 1'b1 to Select TLM Block"]
    #[inline(always)]
    pub fn apb_tlm_sel(&self) -> APB_TLM_SEL_R {
        APB_TLM_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARM firmware/software sets this register 1'b1 to Select TL Block"]
    #[inline(always)]
    pub fn apb_tl_sel(&self) -> APB_TL_SEL_R {
        APB_TL_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ARM firmware/software sets this register 1'b1 to Select BRM Block"]
    #[inline(always)]
    pub fn apb_brm_sel(&self) -> APB_BRM_SEL_R {
        APB_BRM_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ARM firmware/software sets this register 1'b1 to Select BR Block"]
    #[inline(always)]
    pub fn apb_br_sel(&self) -> APB_BR_SEL_R {
        APB_BR_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ARM firmware/software sets this register 1'b1 to Select BLM Block"]
    #[inline(always)]
    pub fn apb_blm_sel(&self) -> APB_BLM_SEL_R {
        APB_BLM_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ARM firmware/software sets this register 1'b1 to Select BL Block"]
    #[inline(always)]
    pub fn apb_bl_sel(&self) -> APB_BL_SEL_R {
        APB_BL_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ARM firmware/software sets this register 1'b1 to enable Partial Load"]
    #[inline(always)]
    pub fn apb_partial_load(&self) -> APB_PARTIAL_LOAD_R {
        APB_PARTIAL_LOAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - ARM firmware/software sets this register to set Word Line Data In"]
    #[inline(always)]
    pub fn apb_wl_din(&self) -> APB_WL_DIN_R {
        APB_WL_DIN_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - ARM firmware/software sets this register 1'b1 and APB_CFG_WR 1'b0 to perform Configuration Read"]
    #[inline(always)]
    pub fn apb_cfg_rd(&self) -> APB_CFG_RD_R {
        APB_CFG_RD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ARM firmware/software sets this register 1'b1 and APB_CFG_RD 1'b0 to perform Configuration Write"]
    #[inline(always)]
    pub fn apb_cfg_wr(&self) -> APB_CFG_WR_R {
        APB_CFG_WR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Shift Register Power Gate Status: 1 in configuration mode, 0 in normal mode"]
    #[inline(always)]
    pub fn sw_pwr_gate(&self) -> SW_PWR_GATE_R {
        SW_PWR_GATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM firmware/software sets this register 1'b1 to Get the Control Right of Cfg_Ctl"]
    #[inline(always)]
    pub fn apb_sel_cfg(&mut self) -> APB_SEL_CFG_W {
        APB_SEL_CFG_W { w: self }
    }
    #[doc = "Bit 1 - ARM firmware/software sets this register 1'b1 to Select TRM Block"]
    #[inline(always)]
    pub fn apb_trm_sel(&mut self) -> APB_TRM_SEL_W {
        APB_TRM_SEL_W { w: self }
    }
    #[doc = "Bit 2 - ARM firmware/software sets this register 1'b1 to Select TR Block"]
    #[inline(always)]
    pub fn apb_tr_sel(&mut self) -> APB_TR_SEL_W {
        APB_TR_SEL_W { w: self }
    }
    #[doc = "Bit 3 - ARM firmware/software sets this register 1'b1 to Select TLM Block"]
    #[inline(always)]
    pub fn apb_tlm_sel(&mut self) -> APB_TLM_SEL_W {
        APB_TLM_SEL_W { w: self }
    }
    #[doc = "Bit 4 - ARM firmware/software sets this register 1'b1 to Select TL Block"]
    #[inline(always)]
    pub fn apb_tl_sel(&mut self) -> APB_TL_SEL_W {
        APB_TL_SEL_W { w: self }
    }
    #[doc = "Bit 5 - ARM firmware/software sets this register 1'b1 to Select BRM Block"]
    #[inline(always)]
    pub fn apb_brm_sel(&mut self) -> APB_BRM_SEL_W {
        APB_BRM_SEL_W { w: self }
    }
    #[doc = "Bit 6 - ARM firmware/software sets this register 1'b1 to Select BR Block"]
    #[inline(always)]
    pub fn apb_br_sel(&mut self) -> APB_BR_SEL_W {
        APB_BR_SEL_W { w: self }
    }
    #[doc = "Bit 7 - ARM firmware/software sets this register 1'b1 to Select BLM Block"]
    #[inline(always)]
    pub fn apb_blm_sel(&mut self) -> APB_BLM_SEL_W {
        APB_BLM_SEL_W { w: self }
    }
    #[doc = "Bit 8 - ARM firmware/software sets this register 1'b1 to Select BL Block"]
    #[inline(always)]
    pub fn apb_bl_sel(&mut self) -> APB_BL_SEL_W {
        APB_BL_SEL_W { w: self }
    }
    #[doc = "Bit 9 - ARM firmware/software sets this register 1'b1 to enable Partial Load"]
    #[inline(always)]
    pub fn apb_partial_load(&mut self) -> APB_PARTIAL_LOAD_W {
        APB_PARTIAL_LOAD_W { w: self }
    }
    #[doc = "Bits 10:13 - ARM firmware/software sets this register to set Word Line Data In"]
    #[inline(always)]
    pub fn apb_wl_din(&mut self) -> APB_WL_DIN_W {
        APB_WL_DIN_W { w: self }
    }
    #[doc = "Bit 14 - ARM firmware/software sets this register 1'b1 and APB_CFG_WR 1'b0 to perform Configuration Read"]
    #[inline(always)]
    pub fn apb_cfg_rd(&mut self) -> APB_CFG_RD_W {
        APB_CFG_RD_W { w: self }
    }
    #[doc = "Bit 15 - ARM firmware/software sets this register 1'b1 and APB_CFG_RD 1'b0 to perform Configuration Write"]
    #[inline(always)]
    pub fn apb_cfg_wr(&mut self) -> APB_CFG_WR_W {
        APB_CFG_WR_W { w: self }
    }
    #[doc = "Bit 31 - Shift Register Power Gate Status: 1 in configuration mode, 0 in normal mode"]
    #[inline(always)]
    pub fn sw_pwr_gate(&mut self) -> SW_PWR_GATE_W {
        SW_PWR_GATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fabric Configuration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_ctl](index.html) module"]
pub struct CFG_CTL_SPEC;
impl crate::RegisterSpec for CFG_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_ctl::R](R) reader structure"]
impl crate::Readable for CFG_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_ctl::W](W) writer structure"]
impl crate::Writable for CFG_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_CTL to value 0x8000_3dfe"]
impl crate::Resettable for CFG_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_3dfe
    }
}
