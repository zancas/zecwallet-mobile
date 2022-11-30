/* eslint-disable react-native/no-inline-styles */
import React from 'react';
import { View, ScrollView, SafeAreaView, Image } from 'react-native';
import { useTheme } from '@react-navigation/native';
import { TranslateOptions } from 'i18n-js';

import RegText from '../Components/RegText';
import ZecAmount from '../Components/ZecAmount';
import BoldText from '../Components/BoldText';
import Button from '../Button';
import DetailLine from './components/DetailLine';
import { ThemeType } from '../../app/types';
import { TotalBalance } from '../../app/AppState';

type PoolsProps = {
  closeModal: () => void;
  totalBalance: TotalBalance;
  currencyName?: string;
  translate: (key: string, config?: TranslateOptions) => any;
};

const Pools: React.FunctionComponent<PoolsProps> = ({ closeModal, totalBalance, currencyName, translate }) => {
  const { colors } = useTheme() as unknown as ThemeType;

  console.log(totalBalance);

  return (
    <SafeAreaView
      style={{
        display: 'flex',
        justifyContent: 'flex-start',
        alignItems: 'stretch',
        height: '100%',
        backgroundColor: colors.background,
      }}>
      <View
        style={{
          display: 'flex',
          alignItems: 'center',
          paddingBottom: 10,
          backgroundColor: colors.card,
          zIndex: -1,
          paddingTop: 10,
        }}>
        <Image
          source={require('../../assets/img/logobig-zingo.png')}
          style={{ width: 80, height: 80, resizeMode: 'contain' }}
        />
        <ZecAmount currencyName={currencyName} size={36} amtZec={totalBalance.total} style={{ opacity: 0.5 }} />
        <RegText color={colors.money} style={{ marginTop: 5, padding: 5 }}>
          {translate('pools.title')}
        </RegText>
        <View style={{ width: '100%', height: 1, backgroundColor: colors.primary }} />
      </View>

      <ScrollView
        style={{ maxHeight: '85%' }}
        contentContainerStyle={{
          flexDirection: 'column',
          alignItems: 'stretch',
          justifyContent: 'flex-start',
        }}>
        <View style={{ display: 'flex', margin: 20 }}>
          <BoldText>{translate('pools.orchard-title')}</BoldText>

          <View style={{ display: 'flex', marginLeft: 25 }}>
            <DetailLine label={translate('pools.orchard-balance')}>
              <ZecAmount amtZec={totalBalance.orchardBal} size={18} currencyName={currencyName} />
            </DetailLine>
            <DetailLine label={translate('pools.orchard-spendable-balance')}>
              <ZecAmount amtZec={totalBalance.spendableOrchard} size={18} currencyName={currencyName} />
            </DetailLine>
          </View>

          <View style={{ height: 1, width: '100%', backgroundColor: 'white', marginTop: 15, marginBottom: 10 }} />

          <BoldText>{translate('pools.sapling-title')}</BoldText>

          <View style={{ display: 'flex', marginLeft: 25 }}>
            <DetailLine label={translate('pools.sapling-balance')}>
              <ZecAmount amtZec={totalBalance.privateBal} size={18} currencyName={currencyName} />
            </DetailLine>
            <DetailLine label={translate('pools.sapling-spendable-balance')}>
              <ZecAmount amtZec={totalBalance.spendablePrivate} size={18} currencyName={currencyName} />
            </DetailLine>
          </View>

          <View style={{ height: 1, width: '100%', backgroundColor: 'white', marginTop: 15, marginBottom: 10 }} />

          <BoldText>{translate('pools.transparent-title')}</BoldText>

          <View style={{ display: 'flex', marginLeft: 25 }}>
            <DetailLine label={translate('pools.transparent-balance')}>
              <ZecAmount amtZec={totalBalance.transparentBal} size={18} currencyName={currencyName} />
            </DetailLine>
          </View>
        </View>
      </ScrollView>

      <View
        style={{
          flexGrow: 1,
          flexDirection: 'row',
          justifyContent: 'center',
          alignItems: 'center',
          marginVertical: 5,
        }}>
        <Button type="Secondary" title={translate('close')} onPress={closeModal} />
      </View>
    </SafeAreaView>
  );
};

export default Pools;
