use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);

        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        // create without parents, check if parents none.
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        // check if exceed MAX value
        // 不能写成：KittiesModule::NextKittyId::<Test>::set(crate::KittyId::max_value());
        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn it_works_breed() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
            Error::<Test>::SameKittyId
        );

        // 测试不存在的kittyId
        assert_noop!(
            KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
            Error::<Test>::InvalidKittyId
        );

        // 创建两只 kitty
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        // 在创建了两个 kitty 后，查看id是否+2了
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        // breed kitty
        assert_ok!(KittiesModule::breed(
            RuntimeOrigin::signed(account_id),
            kitty_id,
            kitty_id + 1
        ));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));

        assert_eq!(
            KittiesModule::kitty_parents(breed_kitty_id),
            Some((kitty_id, kitty_id + 1))
        )
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(KittiesModule::transfer(
            RuntimeOrigin::signed(recipient),
            account_id,
            kitty_id
        ), Error::<Test>::NotOwner);

        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(account_id),
            recipient,
            kitty_id
        ));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));


        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(recipient),
            account_id,
            kitty_id
        ));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
    });
}

#[test]
fn it_works_for_create_event() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        let kitty = KittiesModule::kitties(kitty_id).unwrap();

        System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty }.into());
    });
}

#[test]
fn it_works_for_breed_event() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;

        // create two kitties
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

        // breed a kitty
        assert_ok!(KittiesModule::breed(
            RuntimeOrigin::signed(account_id),
            kitty_id,
            kitty_id + 1
        ));

        // get the newborn kitty id
        let newborn_kitty_id = KittiesModule::next_kitty_id() - 1;
        let newborn_kitty = KittiesModule::kitties(newborn_kitty_id).unwrap();

        System::assert_last_event(Event::KittyBreed { who: account_id, kitty_id: newborn_kitty_id, kitty: newborn_kitty}.into());

        System::assert_has_event(Event::KittyBreed { who: account_id, kitty_id: newborn_kitty_id, kitty: newborn_kitty}.into());
    });
}

#[test]
fn it_works_for_transfer_event() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        // first transfer
        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(account_id),
            recipient,
            kitty_id
        ));

        System::assert_last_event(Event::KittyTransferred { who: account_id, recipient, kitty_id }.into());

        // second transfer
        assert_ok!(KittiesModule::transfer(
            RuntimeOrigin::signed(recipient),
            account_id,
            kitty_id
        ));

        System::assert_last_event(Event::KittyTransferred { who: recipient, recipient: account_id, kitty_id }.into());
    })
}