import { h } from 'vue';
import type { ColumnDef } from '@tanstack/vue-table';
import type { LeadItem } from '@/types/LeadItem';
import DropdownAction from '@/components/leads/DataTableDropDown.vue';
import { ArrowUpDown, ChevronDown } from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Checkbox } from '@/components/ui/checkbox';

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
        accessorKey: 'id',
        header: 'ID',
        cell: ({ row }) => h('div', {}, row.getValue('id')),
    },
    {
        accessorKey: 'name',
        header: 'Name',
        cell: ({ row }) => h('div', {}, row.getValue('name')),
    },
    {
        accessorKey: 'website',
        header: 'Website',
        cell: ({ row }) => h('div', {}, row.getValue('website')),
    },
    {
        accessorKey: 'phone',
        header: 'Phone',
        cell: ({ row }) => h('div', {}, row.getValue('phone')),
    },
    {
        accessorKey: 'address',
        header: 'Address',
        cell: ({ row }) => h('div', {}, row.getValue('address')),
    },
    {
        accessorKey: 'has_website',
        header: 'Has Website',
        cell: ({ row }) => h('div', {}, row.getValue('has_website')),
    },
    {
        accessorKey: 'has_phone',
        header: 'Has Phone',
        cell: ({ row }) => h('div', {}, row.getValue('has_phone')),
    },
    {
        accessorKey: 'total_reviews',
        header: 'Total Reviews',
        cell: ({ row }) => h('div', {}, row.getValue('total_reviews')),
    },
    {
        accessorKey: 'ratings',
        header: 'Ratings',
        cell: ({ row }) => h('div', {}, row.getValue('ratings')),
    },
    {
        accessorKey: 'url',
        header: 'URL',
        cell: ({ row }) => h('a', {
            href: row.getValue('url'),
            target: '_blank', // Open link in a new tab
            rel: 'noopener noreferrer', // Recommended for security when using target="_blank"
        }, row.getValue('url')),
    },

    {
        accessorKey: 'owner_name',
        header: 'Owner Name',
        cell: ({ row }) => h('div', {}, row.getValue('owner_name')),
    },
    {
        accessorKey: 'call_completed',
        header: 'Call Completed',
        cell: ({ row }) => h('div', {}, row.getValue('call_completed')),
    },
    {
        accessorKey: 'call_successful',
        header: 'Call Successful',
        cell: ({ row }) => h('div', {}, row.getValue('call_successful')),
    },
    {
        accessorKey: 'location',
        header: 'Location',
        cell: ({ row }) => h('div', {}, row.getValue('location')),
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
