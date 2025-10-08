import { zodResolver } from '@hookform/resolvers/zod';
import type { HTMLAttributes } from 'react';
import { forwardRef, useImperativeHandle } from 'react';
import type { Control, FieldPath, UseFormReturn } from 'react-hook-form';
import { useForm, useWatch } from 'react-hook-form';
import { useTranslation } from 'react-i18next';

import {
  PROVIDER_OLLAMA,
} from '@/lib/constants';
import {
  editOllamaModelFormSchema,
  newOllamaModelFormSchema,
} from '@/lib/schemas';
import type {
  Model,
  ModelFormHandler,
  NewModel,
  NewOllamaModel,
  OllamaModel,
  RawConfig,
  RawOllamaConfig,
} from '@/lib/types';

import { InputWithMenu } from '../InputWithMenu';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '../ui/form';
import { RemoteModelsSelector } from './RemoteModelsSelector';

type NewFormProps = Omit<HTMLAttributes<HTMLFormElement>, 'onSubmit'> & {
  onSubmit: (model: NewModel) => void;
};

type EditFormProps = Omit<HTMLAttributes<HTMLFormElement>, 'onSubmit'> & {
  model: Model;
  onSubmit: (model: Model) => void;
};

type GenericFormProps<T extends NewModel | Model> = Omit<
  HTMLAttributes<HTMLFormElement>,
  'onSubmit'
> & {
  form: UseFormReturn<T, any, undefined>;
  onSubmit: (model: T) => void;
  loadModelsOnInit?: boolean;
};

type FormFieldProps<T extends NewModel | Model> = {
  control: Control<T>;
  name: FieldPath<T>;
  label: string;
  placeholder?: string;
  tips?: string;
};

// ModelForm's input component
const InputField = <T extends NewModel | Model>({
  control,
  name,
  label,
  placeholder,
  tips,
}: FormFieldProps<T>) => {
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem className="grid grid-cols-4 items-center gap-x-4 gap-y-1 space-y-0">
          <FormLabel className="text-right">{label}</FormLabel>
          <FormControl>
            <InputWithMenu
              className="col-span-3"
              {...field}
              value={(field.value ?? '') as string}
              placeholder={placeholder}
            />
          </FormControl>
          <div className="col-start-2 col-end-4">
            <FormMessage />
            {tips ? <FormDescription>{tips}</FormDescription> : null}
          </div>
        </FormItem>
      )}
    />
  );
};

// ModelForm's hidden input component
const HiddenInputField = <T extends NewModel | Model>({
  control,
  name,
}: Omit<FormFieldProps<T>, 'label' | 'placeholder'>) => {
  return (
    <FormField
      control={control}
      name={name}
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <InputWithMenu
              type="hidden"
              {...field}
              value={(field.value ?? '') as string}
            />
          </FormControl>
          <div className="col-span-4">
            <FormMessage />
          </div>
        </FormItem>
      )}
    />
  );
};

// ModelForm's input for model
const ModelField = <T extends NewModel | Model>({
  control,
  name,
  label,
  tips,
  config,
  loadOnInit = false,
}: Omit<FormFieldProps<T>, 'placeholder'> & {
  config: RawConfig;
  loadOnInit: boolean;
}) => {
  return (
    <div className="grid grid-cols-4 items-center gap-x-4 gap-y-1 space-y-0">
      <FormField
        control={control}
        name={name}
        render={() => <FormLabel className="text-right">{label}</FormLabel>}
      />
      <div className="col-span-3 col-start-2 flex justify-between gap-2">
        <RemoteModelsSelector config={config} enabledByDefault={!!loadOnInit} />
      </div>
      <div className="col-span-3 col-start-2">
        <FormField
          control={control}
          name={name}
          render={() => <FormMessage />}
        />
        <FormDescription>{tips}</FormDescription>
      </div>
    </div>
  );
};

const GenericOllamaModelForm = ({
  form,
  onSubmit,
  loadModelsOnInit,
  ...props
}: GenericFormProps<NewOllamaModel | OllamaModel>) => {
  const { t } = useTranslation(['page-models']);
  const isEdit = !!form.getValues('id');
  const endpoint = useWatch({ name: 'endpoint', control: form.control });
  const config: RawOllamaConfig = {
    provider: PROVIDER_OLLAMA,
    endpoint: endpoint ?? '',
  };
  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} {...props}>
        <div className="grid gap-4 py-8">
          <InputField
            control={form.control}
            name="alias"
            label={t('page-models:label:alias')}
            tips={t('page-models:message:alias-tips')}
          />
          <InputField
            control={form.control}
            name="endpoint"
            label={t('page-models:label:endpoint')}
            tips={t('page-models:message:endpoint-tips')}
          />
          <ModelField
            control={form.control}
            name="model"
            label={t('page-models:label:model')}
            tips={t('page-models:message:model-tips')}
            config={config}
            loadOnInit={!!loadModelsOnInit}
          />
          <HiddenInputField control={form.control} name="provider" />
          {isEdit ? (
            <HiddenInputField control={form.control} name="id" />
          ) : null}
        </div>
      </form>
    </Form>
  );
};

const NewOllamaModelForm = forwardRef<ModelFormHandler, NewFormProps>(
  ({ onSubmit, ...props }, ref) => {
    const form = useForm<NewOllamaModel>({
      resolver: zodResolver(newOllamaModelFormSchema),
      defaultValues: {
        provider: PROVIDER_OLLAMA,
        alias: '',
        endpoint: '',
        model: '',
      },
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOllamaModelForm
        form={form as UseFormReturn<NewOllamaModel, any, undefined>}
        onSubmit={onSubmit}
        {...props}
      />
    );
  }
);

const EditOllamaModelForm = forwardRef<ModelFormHandler, EditFormProps>(
  ({ model, onSubmit, ...props }, ref) => {
    const form = useForm<OllamaModel>({
      resolver: zodResolver(editOllamaModelFormSchema),
      defaultValues: model as OllamaModel,
    });

    useImperativeHandle(ref, () => ({
      reset: () => {
        form.reset();
      },
    }));

    return (
      <GenericOllamaModelForm
        form={form as UseFormReturn<OllamaModel, any, undefined>}
        onSubmit={onSubmit as (model: OllamaModel) => void}
        loadModelsOnInit
        {...props}
      />
    );
  }
);

export default {
  Ollama: {
    New: NewOllamaModelForm,
    Edit: EditOllamaModelForm,
  },
};
