#[doc = "Register `PAD_%s_CTRL` reader"]
pub struct R(crate::R<PAD__CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAD__CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAD__CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAD__CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAD_%s_CTRL` writer"]
pub struct W(crate::W<PAD__CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAD__CTRL_SPEC>;
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
impl From<crate::W<PAD__CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAD__CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Functional selection for IO output. Refer to IO Mux\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_SEL_A {
    #[doc = "0: Select alternative function 0"]
    ALTERNATIVE_0 = 0,
    #[doc = "1: Select alternative function 1"]
    ALTERNATIVE_1 = 1,
    #[doc = "2: Select alternative function 2"]
    ALTERNATIVE_2 = 2,
    #[doc = "3: Select alternative function 3"]
    ALTERNATIVE_3 = 3,
}
impl From<FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FUNC_SEL` reader - Functional selection for IO output. Refer to IO Mux"]
pub struct FUNC_SEL_R(crate::FieldReader<u8, FUNC_SEL_A>);
impl FUNC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUNC_SEL_A {
        match self.bits {
            0 => FUNC_SEL_A::ALTERNATIVE_0,
            1 => FUNC_SEL_A::ALTERNATIVE_1,
            2 => FUNC_SEL_A::ALTERNATIVE_2,
            3 => FUNC_SEL_A::ALTERNATIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALTERNATIVE_0`"]
    #[inline(always)]
    pub fn is_alternative_0(&self) -> bool {
        **self == FUNC_SEL_A::ALTERNATIVE_0
    }
    #[doc = "Checks if the value of the field is `ALTERNATIVE_1`"]
    #[inline(always)]
    pub fn is_alternative_1(&self) -> bool {
        **self == FUNC_SEL_A::ALTERNATIVE_1
    }
    #[doc = "Checks if the value of the field is `ALTERNATIVE_2`"]
    #[inline(always)]
    pub fn is_alternative_2(&self) -> bool {
        **self == FUNC_SEL_A::ALTERNATIVE_2
    }
    #[doc = "Checks if the value of the field is `ALTERNATIVE_3`"]
    #[inline(always)]
    pub fn is_alternative_3(&self) -> bool {
        **self == FUNC_SEL_A::ALTERNATIVE_3
    }
}
impl core::ops::Deref for FUNC_SEL_R {
    type Target = crate::FieldReader<u8, FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUNC_SEL` writer - Functional selection for IO output. Refer to IO Mux"]
pub struct FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUNC_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Select alternative function 0"]
    #[inline(always)]
    pub fn alternative_0(self) -> &'a mut W {
        self.variant(FUNC_SEL_A::ALTERNATIVE_0)
    }
    #[doc = "Select alternative function 1"]
    #[inline(always)]
    pub fn alternative_1(self) -> &'a mut W {
        self.variant(FUNC_SEL_A::ALTERNATIVE_1)
    }
    #[doc = "Select alternative function 2"]
    #[inline(always)]
    pub fn alternative_2(self) -> &'a mut W {
        self.variant(FUNC_SEL_A::ALTERNATIVE_2)
    }
    #[doc = "Select alternative function 3"]
    #[inline(always)]
    pub fn alternative_3(self) -> &'a mut W {
        self.variant(FUNC_SEL_A::ALTERNATIVE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Control selection for IO Output. 0x0 = A0 registers, 0x1 = Others, 0x2 = Fabric\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTRL_SEL_A {
    #[doc = "0: A0 registers"]
    A0_REGISTERS = 0,
    #[doc = "1: Others"]
    OTHERS = 1,
    #[doc = "2: Fabric"]
    FABRIC = 2,
}
impl From<CTRL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTRL_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTRL_SEL` reader - Control selection for IO Output. 0x0 = A0 registers, 0x1 = Others, 0x2 = Fabric"]
pub struct CTRL_SEL_R(crate::FieldReader<u8, CTRL_SEL_A>);
impl CTRL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTRL_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTRL_SEL_A> {
        match self.bits {
            0 => Some(CTRL_SEL_A::A0_REGISTERS),
            1 => Some(CTRL_SEL_A::OTHERS),
            2 => Some(CTRL_SEL_A::FABRIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `A0_REGISTERS`"]
    #[inline(always)]
    pub fn is_a0_registers(&self) -> bool {
        **self == CTRL_SEL_A::A0_REGISTERS
    }
    #[doc = "Checks if the value of the field is `OTHERS`"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        **self == CTRL_SEL_A::OTHERS
    }
    #[doc = "Checks if the value of the field is `FABRIC`"]
    #[inline(always)]
    pub fn is_fabric(&self) -> bool {
        **self == CTRL_SEL_A::FABRIC
    }
}
impl core::ops::Deref for CTRL_SEL_R {
    type Target = crate::FieldReader<u8, CTRL_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRL_SEL` writer - Control selection for IO Output. 0x0 = A0 registers, 0x1 = Others, 0x2 = Fabric"]
pub struct CTRL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "A0 registers"]
    #[inline(always)]
    pub fn a0_registers(self) -> &'a mut W {
        self.variant(CTRL_SEL_A::A0_REGISTERS)
    }
    #[doc = "Others"]
    #[inline(always)]
    pub fn others(self) -> &'a mut W {
        self.variant(CTRL_SEL_A::OTHERS)
    }
    #[doc = "Fabric"]
    #[inline(always)]
    pub fn fabric(self) -> &'a mut W {
        self.variant(CTRL_SEL_A::FABRIC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Active low output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OEN_A {
    #[doc = "0: normal operation"]
    NORMAL_OPERATION = 0,
    #[doc = "1: driver disabled"]
    DRIVER_DISABLED = 1,
}
impl From<OEN_A> for bool {
    #[inline(always)]
    fn from(variant: OEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEN` reader - Active low output enable"]
pub struct OEN_R(crate::FieldReader<bool, OEN_A>);
impl OEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEN_A {
        match self.bits {
            false => OEN_A::NORMAL_OPERATION,
            true => OEN_A::DRIVER_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPERATION`"]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        **self == OEN_A::NORMAL_OPERATION
    }
    #[doc = "Checks if the value of the field is `DRIVER_DISABLED`"]
    #[inline(always)]
    pub fn is_driver_disabled(&self) -> bool {
        **self == OEN_A::DRIVER_DISABLED
    }
}
impl core::ops::Deref for OEN_R {
    type Target = crate::FieldReader<bool, OEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEN` writer - Active low output enable"]
pub struct OEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "normal operation"]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut W {
        self.variant(OEN_A::NORMAL_OPERATION)
    }
    #[doc = "driver disabled"]
    #[inline(always)]
    pub fn driver_disabled(self) -> &'a mut W {
        self.variant(OEN_A::DRIVER_DISABLED)
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
#[doc = "Driver state control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P_A {
    #[doc = "0: floating, high impedance"]
    Z = 0,
    #[doc = "1: pull-up mode"]
    PULL_UP = 1,
    #[doc = "2: pull-down mode"]
    PULL_DOWN = 2,
    #[doc = "3: bus keeper mode"]
    KEEPER = 3,
}
impl From<P_A> for u8 {
    #[inline(always)]
    fn from(variant: P_A) -> Self {
        variant as _
    }
}
#[doc = "Field `P` reader - Driver state control"]
pub struct P_R(crate::FieldReader<u8, P_A>);
impl P_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        P_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P_A {
        match self.bits {
            0 => P_A::Z,
            1 => P_A::PULL_UP,
            2 => P_A::PULL_DOWN,
            3 => P_A::KEEPER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Z`"]
    #[inline(always)]
    pub fn is_z(&self) -> bool {
        **self == P_A::Z
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == P_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == P_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `KEEPER`"]
    #[inline(always)]
    pub fn is_keeper(&self) -> bool {
        **self == P_A::KEEPER
    }
}
impl core::ops::Deref for P_R {
    type Target = crate::FieldReader<u8, P_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P` writer - Driver state control"]
pub struct P_W<'a> {
    w: &'a mut W,
}
impl<'a> P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: P_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "floating, high impedance"]
    #[inline(always)]
    pub fn z(self) -> &'a mut W {
        self.variant(P_A::Z)
    }
    #[doc = "pull-up mode"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(P_A::PULL_UP)
    }
    #[doc = "pull-down mode"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(P_A::PULL_DOWN)
    }
    #[doc = "bus keeper mode"]
    #[inline(always)]
    pub fn keeper(self) -> &'a mut W {
        self.variant(P_A::KEEPER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Driver Strenght\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum E_A {
    #[doc = "0: Configures the drive current at 2mA"]
    CURRENT_2MA = 0,
    #[doc = "1: Configures the drive current at 4mA"]
    CURRENT_4MA = 1,
    #[doc = "2: Configures the drive current at 8mA"]
    CURRENT_8MA = 2,
    #[doc = "3: Configures the drive current at 12mA"]
    CURRENT_12MA = 3,
}
impl From<E_A> for u8 {
    #[inline(always)]
    fn from(variant: E_A) -> Self {
        variant as _
    }
}
#[doc = "Field `E` reader - Driver Strenght"]
pub struct E_R(crate::FieldReader<u8, E_A>);
impl E_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_A {
        match self.bits {
            0 => E_A::CURRENT_2MA,
            1 => E_A::CURRENT_4MA,
            2 => E_A::CURRENT_8MA,
            3 => E_A::CURRENT_12MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CURRENT_2MA`"]
    #[inline(always)]
    pub fn is_current_2ma(&self) -> bool {
        **self == E_A::CURRENT_2MA
    }
    #[doc = "Checks if the value of the field is `CURRENT_4MA`"]
    #[inline(always)]
    pub fn is_current_4ma(&self) -> bool {
        **self == E_A::CURRENT_4MA
    }
    #[doc = "Checks if the value of the field is `CURRENT_8MA`"]
    #[inline(always)]
    pub fn is_current_8ma(&self) -> bool {
        **self == E_A::CURRENT_8MA
    }
    #[doc = "Checks if the value of the field is `CURRENT_12MA`"]
    #[inline(always)]
    pub fn is_current_12ma(&self) -> bool {
        **self == E_A::CURRENT_12MA
    }
}
impl core::ops::Deref for E_R {
    type Target = crate::FieldReader<u8, E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E` writer - Driver Strenght"]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: E_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Configures the drive current at 2mA"]
    #[inline(always)]
    pub fn current_2ma(self) -> &'a mut W {
        self.variant(E_A::CURRENT_2MA)
    }
    #[doc = "Configures the drive current at 4mA"]
    #[inline(always)]
    pub fn current_4ma(self) -> &'a mut W {
        self.variant(E_A::CURRENT_4MA)
    }
    #[doc = "Configures the drive current at 8mA"]
    #[inline(always)]
    pub fn current_8ma(self) -> &'a mut W {
        self.variant(E_A::CURRENT_8MA)
    }
    #[doc = "Configures the drive current at 12mA"]
    #[inline(always)]
    pub fn current_12ma(self) -> &'a mut W {
        self.variant(E_A::CURRENT_12MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Slew Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: slow (half frequency) slew rate"]
    SLOW = 0,
    #[doc = "1: fast slew rate"]
    FAST = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Slew Rate"]
pub struct SR_R(crate::FieldReader<bool, SR_A>);
impl SR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::SLOW,
            true => SR_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        **self == SR_A::SLOW
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == SR_A::FAST
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Slew Rate"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "slow (half frequency) slew rate"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(SR_A::SLOW)
    }
    #[doc = "fast slew rate"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SR_A::FAST)
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
#[doc = "Receive Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REN_A {
    #[doc = "0: disable receive"]
    DISABLE_RECEIVE = 0,
    #[doc = "1: enable receive"]
    ENABLE_RECEIVE = 1,
}
impl From<REN_A> for bool {
    #[inline(always)]
    fn from(variant: REN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REN` reader - Receive Enable"]
pub struct REN_R(crate::FieldReader<bool, REN_A>);
impl REN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REN_A {
        match self.bits {
            false => REN_A::DISABLE_RECEIVE,
            true => REN_A::ENABLE_RECEIVE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_RECEIVE`"]
    #[inline(always)]
    pub fn is_disable_receive(&self) -> bool {
        **self == REN_A::DISABLE_RECEIVE
    }
    #[doc = "Checks if the value of the field is `ENABLE_RECEIVE`"]
    #[inline(always)]
    pub fn is_enable_receive(&self) -> bool {
        **self == REN_A::ENABLE_RECEIVE
    }
}
impl core::ops::Deref for REN_R {
    type Target = crate::FieldReader<bool, REN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REN` writer - Receive Enable"]
pub struct REN_W<'a> {
    w: &'a mut W,
}
impl<'a> REN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable receive"]
    #[inline(always)]
    pub fn disable_receive(self) -> &'a mut W {
        self.variant(REN_A::DISABLE_RECEIVE)
    }
    #[doc = "enable receive"]
    #[inline(always)]
    pub fn enable_receive(self) -> &'a mut W {
        self.variant(REN_A::ENABLE_RECEIVE)
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
#[doc = "Schmitt Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMT_A {
    #[doc = "0: Disable the Schmitt trigger"]
    DISABLE_TRIGGER = 0,
    #[doc = "1: Enable the Schmitt trigger"]
    ENABLE_TRIGGER = 1,
}
impl From<SMT_A> for bool {
    #[inline(always)]
    fn from(variant: SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMT` reader - Schmitt Trigger"]
pub struct SMT_R(crate::FieldReader<bool, SMT_A>);
impl SMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMT_A {
        match self.bits {
            false => SMT_A::DISABLE_TRIGGER,
            true => SMT_A::ENABLE_TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_TRIGGER`"]
    #[inline(always)]
    pub fn is_disable_trigger(&self) -> bool {
        **self == SMT_A::DISABLE_TRIGGER
    }
    #[doc = "Checks if the value of the field is `ENABLE_TRIGGER`"]
    #[inline(always)]
    pub fn is_enable_trigger(&self) -> bool {
        **self == SMT_A::ENABLE_TRIGGER
    }
}
impl core::ops::Deref for SMT_R {
    type Target = crate::FieldReader<bool, SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMT` writer - Schmitt Trigger"]
pub struct SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Schmitt trigger"]
    #[inline(always)]
    pub fn disable_trigger(self) -> &'a mut W {
        self.variant(SMT_A::DISABLE_TRIGGER)
    }
    #[doc = "Enable the Schmitt trigger"]
    #[inline(always)]
    pub fn enable_trigger(self) -> &'a mut W {
        self.variant(SMT_A::ENABLE_TRIGGER)
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
impl R {
    #[doc = "Bits 0:1 - Functional selection for IO output. Refer to IO Mux"]
    #[inline(always)]
    pub fn func_sel(&self) -> FUNC_SEL_R {
        FUNC_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Control selection for IO Output. 0x0 = A0 registers, 0x1 = Others, 0x2 = Fabric"]
    #[inline(always)]
    pub fn ctrl_sel(&self) -> CTRL_SEL_R {
        CTRL_SEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Active low output enable"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Driver state control"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Driver Strenght"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Slew Rate"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Enable"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Schmitt Trigger"]
    #[inline(always)]
    pub fn smt(&self) -> SMT_R {
        SMT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Functional selection for IO output. Refer to IO Mux"]
    #[inline(always)]
    pub fn func_sel(&mut self) -> FUNC_SEL_W {
        FUNC_SEL_W { w: self }
    }
    #[doc = "Bits 3:4 - Control selection for IO Output. 0x0 = A0 registers, 0x1 = Others, 0x2 = Fabric"]
    #[inline(always)]
    pub fn ctrl_sel(&mut self) -> CTRL_SEL_W {
        CTRL_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Active low output enable"]
    #[inline(always)]
    pub fn oen(&mut self) -> OEN_W {
        OEN_W { w: self }
    }
    #[doc = "Bits 6:7 - Driver state control"]
    #[inline(always)]
    pub fn p(&mut self) -> P_W {
        P_W { w: self }
    }
    #[doc = "Bits 8:9 - Driver Strenght"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
    #[doc = "Bit 10 - Slew Rate"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 11 - Receive Enable"]
    #[inline(always)]
    pub fn ren(&mut self) -> REN_W {
        REN_W { w: self }
    }
    #[doc = "Bit 12 - Schmitt Trigger"]
    #[inline(always)]
    pub fn smt(&mut self) -> SMT_W {
        SMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAD_%s control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pad__ctrl](index.html) module"]
pub struct PAD__CTRL_SPEC;
impl crate::RegisterSpec for PAD__CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pad__ctrl::R](R) reader structure"]
impl crate::Readable for PAD__CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pad__ctrl::W](W) writer structure"]
impl crate::Writable for PAD__CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAD_%s_CTRL to value 0"]
impl crate::Resettable for PAD__CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
