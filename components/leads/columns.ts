import { h } from 'vue';
import type { ColumnDef } from '@tanstack/vue-table';
import type { LeadItem } from '@/types/LeadItem';
import DropdownAction from '@/components/leads/DataTableDropDown.vue';
import { CheckIcon, Cross1Icon } from '@radix-icons/vue'
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';
import { Breadcrumb } from '#build/components';

export const columns: ColumnDef<LeadItem>[] = [
    {
        id: 'select',
        header: ({ table }) => h(Checkbox, {
            'checked': table.getIsAllPageRowsSelected(),
            'onUpdate:checked': (value: boolean) => table.toggleAllPageRowsSelected(!!value),
            'ariaLabel': 'Select all',
        }),
        cell: ({ row }) => h(Checkbox, {
            'checked': row.getIsSelected(),
            'onUpdate:checked': (value: boolean) => row.toggleSelected(!!value),
            'ariaLabel': 'Select row',
        }),
        enableSorting: false,
        enableHiding: false,
    },
    {
        accessorKey: 'name',
        header: 'Name',
        cell: ({ row }) => h('div', {}, row.getValue('name')),
    },
    {
        accessorKey: 'website',
        header: 'Website',
        cell: ({ row }) => {
            const websiteValue: string = row.getValue('website');
            if (websiteValue === "Empty") {
                return h(Cross1Icon);
            } else {
                let formattedLink = websiteValue.trim(); // Trim whitespace
                if (!formattedLink.startsWith('http')) {
                    formattedLink = `https://${decodeURIComponent(formattedLink)}`;
                }
                return h("a", {
                    href: formattedLink,
                    target: '_blank', // Open link in a new tab
                    rel: 'noopener noreferrer', // Recommended for security when using target="_blank"
                }, h(Button, { size: "sm", variant: "secondary" },
                row.getValue('website')
                ))
            }
        },
    },

    {
        accessorKey: 'phone',
        header: 'Phone',
        cell: ({ row }) => h("a", {
            href: `tel:${row.getValue('phone')}`,
            variant: "link",
            target: '_blank', // Open link in a new tab
            rel: 'noopener noreferrer', // Recommended for security when using target="_blank"
        }, h(Button, {
            variant: "secondary",
            size: "sm"
        }, row.getValue('phone'))),
    },
    {
        accessorKey: 'address',
        header: 'Address',
        cell: ({ row }) => h('div', {}, row.getValue('address')),
    },
    {
        accessorKey: 'has_website',
        header: "Website?",
        cell: ({ row }) => {
            const hasWebsite = row.getValue('has_website');
            return hasWebsite ? h(CheckIcon) : h(Cross1Icon);
        },
    },
    {
        accessorKey: 'has_phone',
        header: 'Phone?',
        cell: ({ row }) => {
            const hasPhone = row.getValue('has_phone');
            return hasPhone ? h(CheckIcon) : h(Cross1Icon);
        },
    },

    {
        accessorKey: 'ratings',
        header: 'Ratings',
        cell: ({ row }) => h('div', {}, row.getValue('ratings')), enableResizing: true
    },
    {
        accessorKey: 'url',
        header: 'URL',
        cell: ({ row }) => h("a", {
            href: row.getValue('url'),
            variant: "link",
            target: '_blank', // Open link in a new tab
            rel: 'noopener noreferrer', // Recommended for security when using target="_blank"
        }, h(Button, {
            variant: "secondary",
            size: "sm"
        }, 'Open')),
    },

    {
        accessorKey: 'owner_name',
        header: 'Owner Name',
        cell: ({ row }) => h('div', {}, row.getValue('owner_name')),
    },
    {
        accessorKey: 'location',
        header: 'Location',
        cell: ({ row }) => h('div', {
            class:"capitalize"
        }, row.getValue('location')),
    },
    {
        accessorKey: 'status',
        header: 'Status',
        cell: ({ row }) => h('div', {}, row.getValue('status')),
    },
    {
        id: 'actions',
        enableHiding: false,
        cell: ({ row }) => {
            const payment = row.original;

            return h('div', { class: 'relative' }, h(DropdownAction, {
                payment,
            }));
        },
    },
];
