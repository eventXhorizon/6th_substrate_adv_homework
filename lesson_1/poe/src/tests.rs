use super::*;
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// 创建存证
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		// 把vector转为BoundVec
		// let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let claim = vec![0, 1];

		// RuntimeOrigin::signed 构造交易发送方，交易发送方给定为1的原因是mock里面对frame_system模块的配置接口里的AccoundId的类型是u64，所以可以给1这样的整数
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		// 还可以断言链上状态
		assert_eq!(
			Proofs::<Test>::get(&claim),	// 从 use super::*; 在 lib.rs中 引入存储项
			Some((1, frame_system::Pallet::<Test>::block_number()))
		)
	})
}

// 尝试创建一个已存在的存证
#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		// 第一次claim
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
            // 再次进行claim
        	PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
	})
}

// 撤销存证
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
	})
}

// 撤销不存在的存证
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		// 并没有进行存证就直接吊销存证，应该是要失败。没有对链上状态进行更改
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1),claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

// 撤销存证的人不是存证的拥有者
#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		// 调用方不是存证的拥有者
		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2),claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

// 测试转移存证成功
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _  = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));

		let claim = vec![0, 1];
		assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
	})
}

// 测试转移成功后，原存证数据不存在
#[test]
fn transfer_claim_failed_when_claim_no_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		let claim_temp = vec![2, 3];

		assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim_temp.clone(), 2),
            Error::<Test>::ClaimNotExist
        );
	})
}

// 测试转移存证，但转移的发起者非交易发送方
#[test]
fn transfer_claim_failed_not_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

		assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
            Error::<Test>::NotClaimOwner
        );
	})
}

// 测试key超过最大长度
#[test]
fn claim_too_long() {
	new_test_ext().execute_with(|| {
		let claim = vec![1; 100];

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimTooLong
		);
	})
}