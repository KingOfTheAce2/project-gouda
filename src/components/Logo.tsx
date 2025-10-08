/*
 * This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT.
 * See LICENSE and THIRD_PARTY_LICENSES.txt for details.
 */
import React from 'react';
import { Link } from 'react-router-dom';

import logoTxt from '@/assets/images/bear-llm-ai.svg';
import logoImg from '@/assets/images/logo.svg';
import { cn } from '@/lib/utils';

export interface Props {
  expanded?: boolean;
}

export function Logo({ expanded = false }: Props) {
  return (
    <div
      className={cn(
        'flex overflow-hidden w-24',
        expanded ? 'max-w-24' : 'max-w-6'
      )}
    >
      <Link to="/" className="flex h-6 w-full gap-2">
        <img src={logoImg} alt="BEAR LLM AI" />
        <img src={logoTxt} alt="BEAR LLM AI" />
      </Link>
    </div>
  );
}
