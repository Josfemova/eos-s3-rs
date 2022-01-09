#[doc = "Register `PKFB_FIFOCTRL` reader"]
pub struct R(crate::R<PKFB_FIFOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKFB_FIFOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKFB_FIFOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKFB_FIFOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKFB_FIFOCTRL` writer"]
pub struct W(crate::W<PKFB_FIFOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKFB_FIFOCTRL_SPEC>;
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
impl From<crate::W<PKFB_FIFOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKFB_FIFOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set to enable the Packet FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_EN_A {
    #[doc = "0: Disable the packet FIFO"]
    DISABLE = 0,
    #[doc = "1: Enable the packet FIFO"]
    ENABLE = 1,
}
impl From<PF0_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_en` reader - Set to enable the Packet FIFO"]
pub struct PF0_EN_R(crate::FieldReader<bool, PF0_EN_A>);
impl PF0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_EN_A {
        match self.bits {
            false => PF0_EN_A::DISABLE,
            true => PF0_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PF0_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PF0_EN_A::ENABLE
    }
}
impl core::ops::Deref for PF0_EN_R {
    type Target = crate::FieldReader<bool, PF0_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_en` writer - Set to enable the Packet FIFO"]
pub struct PF0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the packet FIFO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF0_EN_A::DISABLE)
    }
    #[doc = "Enable the packet FIFO"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF0_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select which subsystem controls the push operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_PUSH_MUX_A {
    #[doc = "0: Select the M4 subsystem to control the FIFO"]
    M4 = 0,
    #[doc = "1: Select the FFE subsystem to control the FIFO"]
    FFE = 1,
}
impl From<PF0_PUSH_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_PUSH_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_push_mux` reader - Select which subsystem controls the push operation"]
pub struct PF0_PUSH_MUX_R(crate::FieldReader<bool, PF0_PUSH_MUX_A>);
impl PF0_PUSH_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_PUSH_MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_PUSH_MUX_A {
        match self.bits {
            false => PF0_PUSH_MUX_A::M4,
            true => PF0_PUSH_MUX_A::FFE,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        **self == PF0_PUSH_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `FFE`"]
    #[inline(always)]
    pub fn is_ffe(&self) -> bool {
        **self == PF0_PUSH_MUX_A::FFE
    }
}
impl core::ops::Deref for PF0_PUSH_MUX_R {
    type Target = crate::FieldReader<bool, PF0_PUSH_MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_mux` writer - Select which subsystem controls the push operation"]
pub struct PF0_PUSH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_PUSH_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_PUSH_MUX_A::M4)
    }
    #[doc = "Select the FFE subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ffe(self) -> &'a mut W {
        self.variant(PF0_PUSH_MUX_A::FFE)
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
#[doc = "Select which subsystem controls the pop operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_MUX_A {
    #[doc = "0: Select the M4 subsystem to control the FIFO"]
    M4 = 0,
    #[doc = "1: Select the AP subsystem to control the FIFO"]
    AP = 1,
}
impl From<PF0_POP_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_pop_mux` reader - Select which subsystem controls the pop operation"]
pub struct PF0_POP_MUX_R(crate::FieldReader<bool, PF0_POP_MUX_A>);
impl PF0_POP_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_MUX_A {
        match self.bits {
            false => PF0_POP_MUX_A::M4,
            true => PF0_POP_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        **self == PF0_POP_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        **self == PF0_POP_MUX_A::AP
    }
}
impl core::ops::Deref for PF0_POP_MUX_R {
    type Target = crate::FieldReader<bool, PF0_POP_MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_mux` writer - Select which subsystem controls the pop operation"]
pub struct PF0_POP_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_POP_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_POP_MUX_A::AP)
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
#[doc = "Select which subsystem manages the push interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_PUSH_INT_MUX_A {
    #[doc = "0: Select the M4 subsystem to control the FIFO"]
    M4 = 0,
    #[doc = "1: Select the AP subsystem to control the FIFO"]
    AP = 1,
}
impl From<PF0_PUSH_INT_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_PUSH_INT_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_push_int_mux` reader - Select which subsystem manages the push interrupt"]
pub struct PF0_PUSH_INT_MUX_R(crate::FieldReader<bool, PF0_PUSH_INT_MUX_A>);
impl PF0_PUSH_INT_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_PUSH_INT_MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_PUSH_INT_MUX_A {
        match self.bits {
            false => PF0_PUSH_INT_MUX_A::M4,
            true => PF0_PUSH_INT_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        **self == PF0_PUSH_INT_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        **self == PF0_PUSH_INT_MUX_A::AP
    }
}
impl core::ops::Deref for PF0_PUSH_INT_MUX_R {
    type Target = crate::FieldReader<bool, PF0_PUSH_INT_MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_push_int_mux` writer - Select which subsystem manages the push interrupt"]
pub struct PF0_PUSH_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_PUSH_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_PUSH_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_PUSH_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_PUSH_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Select which subsystem manages the pop interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_POP_INT_MUX_A {
    #[doc = "0: Select the M4 subsystem to control the FIFO"]
    M4 = 0,
    #[doc = "1: Select the AP subsystem to control the FIFO"]
    AP = 1,
}
impl From<PF0_POP_INT_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_POP_INT_MUX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_pop_int_mux` reader - Select which subsystem manages the pop interrupt"]
pub struct PF0_POP_INT_MUX_R(crate::FieldReader<bool, PF0_POP_INT_MUX_A>);
impl PF0_POP_INT_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_POP_INT_MUX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_POP_INT_MUX_A {
        match self.bits {
            false => PF0_POP_INT_MUX_A::M4,
            true => PF0_POP_INT_MUX_A::AP,
        }
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        **self == PF0_POP_INT_MUX_A::M4
    }
    #[doc = "Checks if the value of the field is `AP`"]
    #[inline(always)]
    pub fn is_ap(&self) -> bool {
        **self == PF0_POP_INT_MUX_A::AP
    }
}
impl core::ops::Deref for PF0_POP_INT_MUX_R {
    type Target = crate::FieldReader<bool, PF0_POP_INT_MUX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_pop_int_mux` writer - Select which subsystem manages the pop interrupt"]
pub struct PF0_POP_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_POP_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_POP_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF0_POP_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF0_POP_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "If FFE is the controller, select which instance of FFE will be used for control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PF0_FFE_SEL_A {
    #[doc = "0: when FFE is the controller of the FIFO, select FFE0 as the controller"]
    FFE0 = 0,
    #[doc = "1: when FFE is the controller of the FIFO, select FFE1 as the controller"]
    FFE1 = 1,
}
impl From<PF0_FFE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PF0_FFE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pf0_ffe_sel` reader - If FFE is the controller, select which instance of FFE will be used for control"]
pub struct PF0_FFE_SEL_R(crate::FieldReader<bool, PF0_FFE_SEL_A>);
impl PF0_FFE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PF0_FFE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PF0_FFE_SEL_A {
        match self.bits {
            false => PF0_FFE_SEL_A::FFE0,
            true => PF0_FFE_SEL_A::FFE1,
        }
    }
    #[doc = "Checks if the value of the field is `FFE0`"]
    #[inline(always)]
    pub fn is_ffe0(&self) -> bool {
        **self == PF0_FFE_SEL_A::FFE0
    }
    #[doc = "Checks if the value of the field is `FFE1`"]
    #[inline(always)]
    pub fn is_ffe1(&self) -> bool {
        **self == PF0_FFE_SEL_A::FFE1
    }
}
impl core::ops::Deref for PF0_FFE_SEL_R {
    type Target = crate::FieldReader<bool, PF0_FFE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pf0_ffe_sel` writer - If FFE is the controller, select which instance of FFE will be used for control"]
pub struct PF0_FFE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PF0_FFE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF0_FFE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE0 as the controller"]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(PF0_FFE_SEL_A::FFE0)
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE1 as the controller"]
    #[inline(always)]
    pub fn ffe1(self) -> &'a mut W {
        self.variant(PF0_FFE_SEL_A::FFE1)
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
#[doc = "Set to enable the Packet FIFO"]
pub type PF1_EN_A = PF0_EN_A;
#[doc = "Field `pf1_en` reader - Set to enable the Packet FIFO"]
pub type PF1_EN_R = PF0_EN_R;
#[doc = "Field `pf1_en` writer - Set to enable the Packet FIFO"]
pub struct PF1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the packet FIFO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF1_EN_A::DISABLE)
    }
    #[doc = "Enable the packet FIFO"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF1_EN_A::ENABLE)
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
            (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Select which subsystem controls the push operation"]
pub type PF1_PUSH_MUX_A = PF0_PUSH_MUX_A;
#[doc = "Field `pf1_push_mux` reader - Select which subsystem controls the push operation"]
pub type PF1_PUSH_MUX_R = PF0_PUSH_MUX_R;
#[doc = "Field `pf1_push_mux` writer - Select which subsystem controls the push operation"]
pub struct PF1_PUSH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_PUSH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_PUSH_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF1_PUSH_MUX_A::M4)
    }
    #[doc = "Select the FFE subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ffe(self) -> &'a mut W {
        self.variant(PF1_PUSH_MUX_A::FFE)
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
            (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Select which subsystem controls the pop operation"]
pub type PF1_POP_MUX_A = PF0_POP_MUX_A;
#[doc = "Field `pf1_pop_mux` reader - Select which subsystem controls the pop operation"]
pub type PF1_POP_MUX_R = PF0_POP_MUX_R;
#[doc = "Field `pf1_pop_mux` writer - Select which subsystem controls the pop operation"]
pub struct PF1_POP_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_POP_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_POP_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF1_POP_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF1_POP_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Select which subsystem manages the push interrupt"]
pub type PF1_PUSH_INT_MUX_A = PF0_PUSH_INT_MUX_A;
#[doc = "Field `pf1_push_int_mux` reader - Select which subsystem manages the push interrupt"]
pub type PF1_PUSH_INT_MUX_R = PF0_PUSH_INT_MUX_R;
#[doc = "Field `pf1_push_int_mux` writer - Select which subsystem manages the push interrupt"]
pub struct PF1_PUSH_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_PUSH_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_PUSH_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF1_PUSH_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF1_PUSH_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Select which subsystem manages the pop interrupt"]
pub type PF1_POP_INT_MUX_A = PF0_POP_INT_MUX_A;
#[doc = "Field `pf1_pop_int_mux` reader - Select which subsystem manages the pop interrupt"]
pub type PF1_POP_INT_MUX_R = PF0_POP_INT_MUX_R;
#[doc = "Field `pf1_pop_int_mux` writer - Select which subsystem manages the pop interrupt"]
pub struct PF1_POP_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_POP_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_POP_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF1_POP_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF1_POP_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF1_FFE_SEL_A = PF0_FFE_SEL_A;
#[doc = "Field `pf1_ffe_sel` reader - If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF1_FFE_SEL_R = PF0_FFE_SEL_R;
#[doc = "Field `pf1_ffe_sel` writer - If FFE is the controller, select which instance of FFE will be used for control"]
pub struct PF1_FFE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PF1_FFE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF1_FFE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE0 as the controller"]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(PF1_FFE_SEL_A::FFE0)
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE1 as the controller"]
    #[inline(always)]
    pub fn ffe1(self) -> &'a mut W {
        self.variant(PF1_FFE_SEL_A::FFE1)
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
            (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Set to enable the Packet FIFO"]
pub type PF2_EN_A = PF0_EN_A;
#[doc = "Field `pf2_en` reader - Set to enable the Packet FIFO"]
pub type PF2_EN_R = PF0_EN_R;
#[doc = "Field `pf2_en` writer - Set to enable the Packet FIFO"]
pub struct PF2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the packet FIFO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF2_EN_A::DISABLE)
    }
    #[doc = "Enable the packet FIFO"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF2_EN_A::ENABLE)
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
            (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Select which subsystem controls the push operation"]
pub type PF2_PUSH_MUX_A = PF0_PUSH_MUX_A;
#[doc = "Field `pf2_push_mux` reader - Select which subsystem controls the push operation"]
pub type PF2_PUSH_MUX_R = PF0_PUSH_MUX_R;
#[doc = "Field `pf2_push_mux` writer - Select which subsystem controls the push operation"]
pub struct PF2_PUSH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_PUSH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_PUSH_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF2_PUSH_MUX_A::M4)
    }
    #[doc = "Select the FFE subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ffe(self) -> &'a mut W {
        self.variant(PF2_PUSH_MUX_A::FFE)
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
            (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Select which subsystem controls the pop operation"]
pub type PF2_POP_MUX_A = PF0_POP_MUX_A;
#[doc = "Field `pf2_pop_mux` reader - Select which subsystem controls the pop operation"]
pub type PF2_POP_MUX_R = PF0_POP_MUX_R;
#[doc = "Field `pf2_pop_mux` writer - Select which subsystem controls the pop operation"]
pub struct PF2_POP_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_POP_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_POP_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF2_POP_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF2_POP_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Select which subsystem manages the push interrupt"]
pub type PF2_PUSH_INT_MUX_A = PF0_PUSH_INT_MUX_A;
#[doc = "Field `pf2_push_int_mux` reader - Select which subsystem manages the push interrupt"]
pub type PF2_PUSH_INT_MUX_R = PF0_PUSH_INT_MUX_R;
#[doc = "Field `pf2_push_int_mux` writer - Select which subsystem manages the push interrupt"]
pub struct PF2_PUSH_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_PUSH_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_PUSH_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF2_PUSH_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF2_PUSH_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Select which subsystem manages the pop interrupt"]
pub type PF2_POP_INT_MUX_A = PF0_POP_INT_MUX_A;
#[doc = "Field `pf2_pop_int_mux` reader - Select which subsystem manages the pop interrupt"]
pub type PF2_POP_INT_MUX_R = PF0_POP_INT_MUX_R;
#[doc = "Field `pf2_pop_int_mux` writer - Select which subsystem manages the pop interrupt"]
pub struct PF2_POP_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_POP_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_POP_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF2_POP_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF2_POP_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF2_FFE_SEL_A = PF0_FFE_SEL_A;
#[doc = "Field `pf2_ffe_sel` reader - If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF2_FFE_SEL_R = PF0_FFE_SEL_R;
#[doc = "Field `pf2_ffe_sel` writer - If FFE is the controller, select which instance of FFE will be used for control"]
pub struct PF2_FFE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PF2_FFE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF2_FFE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE0 as the controller"]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(PF2_FFE_SEL_A::FFE0)
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE1 as the controller"]
    #[inline(always)]
    pub fn ffe1(self) -> &'a mut W {
        self.variant(PF2_FFE_SEL_A::FFE1)
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
            (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Set to enable the Packet FIFO"]
pub type PF8K_EN_A = PF0_EN_A;
#[doc = "Field `pf8k_en` reader - Set to enable the Packet FIFO"]
pub type PF8K_EN_R = PF0_EN_R;
#[doc = "Field `pf8k_en` writer - Set to enable the Packet FIFO"]
pub struct PF8K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the packet FIFO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PF8K_EN_A::DISABLE)
    }
    #[doc = "Enable the packet FIFO"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PF8K_EN_A::ENABLE)
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
            (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Select which subsystem controls the push operation"]
pub type PF8K_PUSH_MUX_A = PF0_PUSH_MUX_A;
#[doc = "Field `pf8k_push_mux` reader - Select which subsystem controls the push operation"]
pub type PF8K_PUSH_MUX_R = PF0_PUSH_MUX_R;
#[doc = "Field `pf8k_push_mux` writer - Select which subsystem controls the push operation"]
pub struct PF8K_PUSH_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_PUSH_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF8K_PUSH_MUX_A::M4)
    }
    #[doc = "Select the FFE subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ffe(self) -> &'a mut W {
        self.variant(PF8K_PUSH_MUX_A::FFE)
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
            (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Select which subsystem controls the pop operation"]
pub type PF8K_POP_MUX_A = PF0_POP_MUX_A;
#[doc = "Field `pf8k_pop_mux` reader - Select which subsystem controls the pop operation"]
pub type PF8K_POP_MUX_R = PF0_POP_MUX_R;
#[doc = "Field `pf8k_pop_mux` writer - Select which subsystem controls the pop operation"]
pub struct PF8K_POP_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_POP_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_POP_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF8K_POP_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF8K_POP_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Select which subsystem manages the push interrupt"]
pub type PF8K_PUSH_INT_MUX_A = PF0_PUSH_INT_MUX_A;
#[doc = "Field `pf8k_push_int_mux` reader - Select which subsystem manages the push interrupt"]
pub type PF8K_PUSH_INT_MUX_R = PF0_PUSH_INT_MUX_R;
#[doc = "Field `pf8k_push_int_mux` writer - Select which subsystem manages the push interrupt"]
pub struct PF8K_PUSH_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_PUSH_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_PUSH_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF8K_PUSH_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF8K_PUSH_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Select which subsystem manages the pop interrupt"]
pub type PF8K_POP_INT_MUX_A = PF0_POP_INT_MUX_A;
#[doc = "Field `pf8k_pop_int_mux` reader - Select which subsystem manages the pop interrupt"]
pub type PF8K_POP_INT_MUX_R = PF0_POP_INT_MUX_R;
#[doc = "Field `pf8k_pop_int_mux` writer - Select which subsystem manages the pop interrupt"]
pub struct PF8K_POP_INT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_POP_INT_MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_POP_INT_MUX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Select the M4 subsystem to control the FIFO"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(PF8K_POP_INT_MUX_A::M4)
    }
    #[doc = "Select the AP subsystem to control the FIFO"]
    #[inline(always)]
    pub fn ap(self) -> &'a mut W {
        self.variant(PF8K_POP_INT_MUX_A::AP)
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
            (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF8K_FFE_SEL_A = PF0_FFE_SEL_A;
#[doc = "Field `pf8k_ffe_sel` reader - If FFE is the controller, select which instance of FFE will be used for control"]
pub type PF8K_FFE_SEL_R = PF0_FFE_SEL_R;
#[doc = "Field `pf8k_ffe_sel` writer - If FFE is the controller, select which instance of FFE will be used for control"]
pub struct PF8K_FFE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PF8K_FFE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PF8K_FFE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE0 as the controller"]
    #[inline(always)]
    pub fn ffe0(self) -> &'a mut W {
        self.variant(PF8K_FFE_SEL_A::FFE0)
    }
    #[doc = "when FFE is the controller of the FIFO, select FFE1 as the controller"]
    #[inline(always)]
    pub fn ffe1(self) -> &'a mut W {
        self.variant(PF8K_FFE_SEL_A::FFE1)
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
            (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf0_en(&self) -> PF0_EN_R {
        PF0_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf0_push_mux(&self) -> PF0_PUSH_MUX_R {
        PF0_PUSH_MUX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf0_pop_mux(&self) -> PF0_POP_MUX_R {
        PF0_POP_MUX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf0_push_int_mux(&self) -> PF0_PUSH_INT_MUX_R {
        PF0_PUSH_INT_MUX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf0_pop_int_mux(&self) -> PF0_POP_INT_MUX_R {
        PF0_POP_INT_MUX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf0_ffe_sel(&self) -> PF0_FFE_SEL_R {
        PF0_FFE_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf1_en(&self) -> PF1_EN_R {
        PF1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf1_push_mux(&self) -> PF1_PUSH_MUX_R {
        PF1_PUSH_MUX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf1_pop_mux(&self) -> PF1_POP_MUX_R {
        PF1_POP_MUX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf1_push_int_mux(&self) -> PF1_PUSH_INT_MUX_R {
        PF1_PUSH_INT_MUX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf1_pop_int_mux(&self) -> PF1_POP_INT_MUX_R {
        PF1_POP_INT_MUX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf1_ffe_sel(&self) -> PF1_FFE_SEL_R {
        PF1_FFE_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf2_en(&self) -> PF2_EN_R {
        PF2_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf2_push_mux(&self) -> PF2_PUSH_MUX_R {
        PF2_PUSH_MUX_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf2_pop_mux(&self) -> PF2_POP_MUX_R {
        PF2_POP_MUX_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf2_push_int_mux(&self) -> PF2_PUSH_INT_MUX_R {
        PF2_PUSH_INT_MUX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf2_pop_int_mux(&self) -> PF2_POP_INT_MUX_R {
        PF2_POP_INT_MUX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf2_ffe_sel(&self) -> PF2_FFE_SEL_R {
        PF2_FFE_SEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf8k_en(&self) -> PF8K_EN_R {
        PF8K_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf8k_push_mux(&self) -> PF8K_PUSH_MUX_R {
        PF8K_PUSH_MUX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf8k_pop_mux(&self) -> PF8K_POP_MUX_R {
        PF8K_POP_MUX_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf8k_push_int_mux(&self) -> PF8K_PUSH_INT_MUX_R {
        PF8K_PUSH_INT_MUX_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf8k_pop_int_mux(&self) -> PF8K_POP_INT_MUX_R {
        PF8K_POP_INT_MUX_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf8k_ffe_sel(&self) -> PF8K_FFE_SEL_R {
        PF8K_FFE_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf0_en(&mut self) -> PF0_EN_W {
        PF0_EN_W { w: self }
    }
    #[doc = "Bit 1 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf0_push_mux(&mut self) -> PF0_PUSH_MUX_W {
        PF0_PUSH_MUX_W { w: self }
    }
    #[doc = "Bit 2 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf0_pop_mux(&mut self) -> PF0_POP_MUX_W {
        PF0_POP_MUX_W { w: self }
    }
    #[doc = "Bit 3 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf0_push_int_mux(&mut self) -> PF0_PUSH_INT_MUX_W {
        PF0_PUSH_INT_MUX_W { w: self }
    }
    #[doc = "Bit 4 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf0_pop_int_mux(&mut self) -> PF0_POP_INT_MUX_W {
        PF0_POP_INT_MUX_W { w: self }
    }
    #[doc = "Bit 5 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf0_ffe_sel(&mut self) -> PF0_FFE_SEL_W {
        PF0_FFE_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf1_en(&mut self) -> PF1_EN_W {
        PF1_EN_W { w: self }
    }
    #[doc = "Bit 9 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf1_push_mux(&mut self) -> PF1_PUSH_MUX_W {
        PF1_PUSH_MUX_W { w: self }
    }
    #[doc = "Bit 10 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf1_pop_mux(&mut self) -> PF1_POP_MUX_W {
        PF1_POP_MUX_W { w: self }
    }
    #[doc = "Bit 11 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf1_push_int_mux(&mut self) -> PF1_PUSH_INT_MUX_W {
        PF1_PUSH_INT_MUX_W { w: self }
    }
    #[doc = "Bit 12 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf1_pop_int_mux(&mut self) -> PF1_POP_INT_MUX_W {
        PF1_POP_INT_MUX_W { w: self }
    }
    #[doc = "Bit 13 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf1_ffe_sel(&mut self) -> PF1_FFE_SEL_W {
        PF1_FFE_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf2_en(&mut self) -> PF2_EN_W {
        PF2_EN_W { w: self }
    }
    #[doc = "Bit 17 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf2_push_mux(&mut self) -> PF2_PUSH_MUX_W {
        PF2_PUSH_MUX_W { w: self }
    }
    #[doc = "Bit 18 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf2_pop_mux(&mut self) -> PF2_POP_MUX_W {
        PF2_POP_MUX_W { w: self }
    }
    #[doc = "Bit 19 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf2_push_int_mux(&mut self) -> PF2_PUSH_INT_MUX_W {
        PF2_PUSH_INT_MUX_W { w: self }
    }
    #[doc = "Bit 20 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf2_pop_int_mux(&mut self) -> PF2_POP_INT_MUX_W {
        PF2_POP_INT_MUX_W { w: self }
    }
    #[doc = "Bit 21 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf2_ffe_sel(&mut self) -> PF2_FFE_SEL_W {
        PF2_FFE_SEL_W { w: self }
    }
    #[doc = "Bit 24 - Set to enable the Packet FIFO"]
    #[inline(always)]
    pub fn pf8k_en(&mut self) -> PF8K_EN_W {
        PF8K_EN_W { w: self }
    }
    #[doc = "Bit 25 - Select which subsystem controls the push operation"]
    #[inline(always)]
    pub fn pf8k_push_mux(&mut self) -> PF8K_PUSH_MUX_W {
        PF8K_PUSH_MUX_W { w: self }
    }
    #[doc = "Bit 26 - Select which subsystem controls the pop operation"]
    #[inline(always)]
    pub fn pf8k_pop_mux(&mut self) -> PF8K_POP_MUX_W {
        PF8K_POP_MUX_W { w: self }
    }
    #[doc = "Bit 27 - Select which subsystem manages the push interrupt"]
    #[inline(always)]
    pub fn pf8k_push_int_mux(&mut self) -> PF8K_PUSH_INT_MUX_W {
        PF8K_PUSH_INT_MUX_W { w: self }
    }
    #[doc = "Bit 28 - Select which subsystem manages the pop interrupt"]
    #[inline(always)]
    pub fn pf8k_pop_int_mux(&mut self) -> PF8K_POP_INT_MUX_W {
        PF8K_POP_INT_MUX_W { w: self }
    }
    #[doc = "Bit 29 - If FFE is the controller, select which instance of FFE will be used for control"]
    #[inline(always)]
    pub fn pf8k_ffe_sel(&mut self) -> PF8K_FFE_SEL_W {
        PF8K_FFE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet FIFO Bank control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkfb_fifoctrl](index.html) module"]
pub struct PKFB_FIFOCTRL_SPEC;
impl crate::RegisterSpec for PKFB_FIFOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkfb_fifoctrl::R](R) reader structure"]
impl crate::Readable for PKFB_FIFOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkfb_fifoctrl::W](W) writer structure"]
impl crate::Writable for PKFB_FIFOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PKFB_FIFOCTRL to value 0"]
impl crate::Resettable for PKFB_FIFOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
